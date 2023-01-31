use async_trait::async_trait;
use bitcoin::{consensus::encode, util::psbt::PartiallySignedTransaction};
use serde::{Deserialize, Serialize};
use tonic_lnd::walletrpc::SignPsbtRequest;

use std::fs;

use super::{error::*, r#trait::*};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LndSignerConfig {
    pub endpoint: String,
    pub cert_base64: String,
    pub macaroon_base64: String,
}

pub struct LndRemoteSigner {
    inner: tonic_lnd::Client,
}

impl LndRemoteSigner {
    pub async fn connect(cfg: LndSignerConfig) -> Result<Self, SigningClientError> {
        use std::{io::Write, os::unix::fs::OpenOptionsExt};
        let tmpdir = tempfile::tempdir()?;
        let cert_file = tmpdir.path().join("cert");

        let mut cert = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .mode(0o600)
            .open(&cert_file)?;
        cert.write_all(&base64::decode(&cfg.cert_base64)?)?;
        let macaroon_file = tmpdir.path().join("macaroon");
        let mut macaroon = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .mode(0o600)
            .open(&macaroon_file)?;
        macaroon.write_all(&base64::decode(&cfg.macaroon_base64)?)?;
        let client = tonic_lnd::connect(cfg.endpoint, cert_file, macaroon_file)
            .await
            .map_err(|e| {
                SigningClientError::CouldNotConnect(format!("Failed to connect to lnd: {}", e))
            })?;
        Ok(Self { inner: client })
    }
}

#[async_trait]
impl RemoteSigningClient for LndRemoteSigner {
    async fn sign_psbt(
        &mut self,
        psbt: &PartiallySignedTransaction,
    ) -> Result<PartiallySignedTransaction, SigningClientError> {
        let response = self
            .inner
            .wallet()
            .sign_psbt(SignPsbtRequest {
                funded_psbt: encode::serialize(psbt),
            })
            .await
            .map_err(|e| {
                SigningClientError::RemoteCallFailure(format!("Failed to sign psbt via lnd: {}", e))
            })?;
        let signed_psbt = response.into_inner().signed_psbt;
        Ok(encode::deserialize(&signed_psbt)?)
    }
}