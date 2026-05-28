use soroban_sdk::{contracttype, Address, BytesN, Env};

// ----------------------------------------------------------------
// Status enum — tracks lifecycle of invoice
// ----------------------------------------------------------------

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum InvoiceStatus {
    Pending,         // submitted, waiting for a liquidity provider to fund it
    Funded,          // LP has funded it, freelancer has been paid out
    PartiallyFunded, // partially funded by one or more LPs
    Paid,            // payer has settled in full, LP has been released
    Defaulted,       // past due_date and still unpaid
    Appealed,        // payer has contested the default ruling (issue #36)
    Expired,         // past due_date with no funding
    Cancelled,       // freelancer cancelled the invoice before funding
}

// ----------------------------------------------------------------
// Invoice struct (UPDATED - token stays per invoice)
// ----------------------------------------------------------------

#[contracttype]
#[derive(Clone, Debug)]
pub struct Invoice {
    pub id: u64,
    pub freelancer: Address, // who submitted the invoice (receives liquidity)
    pub payer: Address,      // the client who owes the money
    pub token: Address,      // token used for this invoice lifecycle
    pub amount: i128,        // full invoice value in stroops (1 USDC = 10_000_000)
    pub due_date: u64,       // Unix timestamp — when the payer must settle by
    pub discount_rate: u32,  // basis points, e.g. 300 = 3.00%
    pub status: InvoiceStatus,
    pub funder: Option<Address>, // set when an LP funds the invoice (legacy for full funding)
    pub funded_at: Option<u64>,  // ledger timestamp when funding occurred
    pub amount_funded: i128,     // cumulative amount funded so far
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct InvoiceParams {
    pub freelancer: Address,
    pub payer: Address,
    pub amount: i128,
    pub due_date: u64,
    pub discount_rate: u32,
    pub token: Address,
}

#[contracttype]
#[derive(Clone, Debug, Default)]
pub struct PayerStats {
    pub total_invoices: u64,
    pub paid_on_time: u64,
    pub defaults: u64,
    pub total_volume: i128,
}

#[contracttype]
#[derive(Clone, Debug, Default)]
pub struct ContractStats {
    pub total_invoices: u64,
    pub total_funded: u64,
    pub total_paid: u64,
    pub total_volume_usdc: i128,
    pub total_volume_eurc: i128,
    pub total_volume_xlm: i128,
}

// ----------------------------------------------------------------
// Issue #36: Appeal record stored per invoice
// ----------------------------------------------------------------

#[contracttype]
#[derive(Clone, Debug)]
pub struct AppealRecord {
    /// SHA-256 hash of off-chain evidence submitted by the payer.
    pub evidence_hash: BytesN<32>,
    /// Ledger timestamp when the appeal was filed.
    pub appealed_at: u64,
    /// Payer reputation score just before the default was applied,
    /// used to restore the score if the appeal is upheld.
    pub pre_default_score: u32,
}

// ----------------------------------------------------------------
// Issue #34: Single entry in the LP priority queue
// ----------------------------------------------------------------

#[contracttype]
#[derive(Clone, Debug)]
pub struct LpFundRequest {
    pub lp: Address,
    /// LP reputation score snapshotted at request time (used for ordering).
    pub score: u32,
}

// ----------------------------------------------------------------

// ----------------------------------------------------------------
// Reputation Score
// ----------------------------------------------------------------

#[contracttype]
#[derive(Clone, Debug, Default)]
pub struct ReputationScore {
    pub score: u32,
    pub last_activity_ledger: u64,
}
