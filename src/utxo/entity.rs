use crate::primitives::{bitcoin::*, *};
use derive_builder::Builder;

pub struct WalletUtxo {
    pub wallet_id: WalletId,
    pub keychain_id: KeychainId,
    pub outpoint: OutPoint,
    pub kind: KeychainKind,
    pub address_idx: u32,
    pub value: Satoshis,
    pub address: Option<bitcoin::Address>,
    pub bdk_spent: bool,
    pub block_height: Option<u32>,
    pub utxo_detected_ledger_tx_id: LedgerTransactionId,
    pub utxo_settled_ledger_tx_id: Option<LedgerTransactionId>,
    pub spending_batch_id: Option<BatchId>,
}

#[derive(Debug)]
pub struct SettledUtxo {
    pub keychain_id: KeychainId,
    pub value: Satoshis,
    pub address: bitcoin::Address,
    pub utxo_detected_ledger_tx_id: LedgerTransactionId,
    pub utxo_settled_ledger_tx_id: LedgerTransactionId,
    pub spend_detected_ledger_tx_id: Option<LedgerTransactionId>,
}

#[derive(Debug)]
pub(super) struct SpentUtxo {
    pub outpoint: bitcoin::OutPoint,
    pub value: Satoshis,
    pub change_address: bool,
    pub confirmed: bool,
}

pub struct KeychainUtxos {
    pub keychain_id: KeychainId,
    pub utxos: Vec<WalletUtxo>,
}

#[derive(Builder)]
pub struct NewUtxo {
    pub(super) account_id: AccountId,
    pub(super) wallet_id: WalletId,
    pub(super) keychain_id: KeychainId,
    pub(super) outpoint: OutPoint,
    pub(super) kind: KeychainKind,
    pub(super) address_idx: u32,
    #[builder(setter(into))]
    pub(super) value: Satoshis,
    pub(super) address: String,
    pub(super) script_hex: String,
    pub(super) sats_per_vbyte_when_created: f32,
    pub(super) self_pay: bool,
    pub(super) bdk_spent: bool,
    pub(super) utxo_detected_ledger_tx_id: LedgerTransactionId,
}

impl NewUtxo {
    pub fn builder() -> NewUtxoBuilder {
        let mut builder = NewUtxoBuilder::default();
        builder.utxo_detected_ledger_tx_id(LedgerTransactionId::new());
        builder
    }
}
