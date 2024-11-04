use anchor_lang::prelude::*;

declare_id!("13gDzEXCdocbj8iAiqrScGo47NiSuYENGsRqi3SEAwet");

#[program]
mod adrena_abi {
    // use super::*;
}

#[account]
#[derive(Debug)]
pub struct Thread {
    pub authority: Pubkey,
    pub bump: u8,
    pub created_at: ClockData,
    pub domain: Option<Vec<u8>>,
    pub exec_context: Option<ExecContext>,
    pub fee: u64,
    pub id: Vec<u8>,
    pub instructions: Vec<SerializableInstruction>,
    pub next_instruction: Option<SerializableInstruction>,
    pub paused: bool,
    pub rate_limit: u64,
    pub trigger: Trigger,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, PartialEq)]
pub struct ClockData {
    pub slot: u64,
    pub epoch: u64,
    pub unix_timestamp: i64,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecContext {
    pub exec_index: u64,
    pub execs_since_reimbursement: u64,
    pub execs_since_slot: u64,
    pub last_exec_at: u64,
    pub trigger_context: TriggerContext,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, PartialEq)]
pub enum Trigger {
    Account {
        address: Pubkey,
        offset: u64,
        size: u64,
    },
    Cron {
        schedule: String,
        skippable: bool,
    },
    Now,
    Slot {
        slot: u64,
    },
    Epoch {
        epoch: u64,
    },
    Timestamp {
        unix_ts: i64,
    },
    Pyth {
        feed_id: FeedId,
        equality: Equality,
        limit: i64,
    },
    Periodic {
        delay: u64,
    },
}

pub type FeedId = [u8; 32];

#[repr(u8)]
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Equality {
    GreaterThanOrEqual,
    LessThanOrEqual,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TriggerContext {
    Account { data_hash: u64 },
    Cron { started_at: i64 },
    Now,
    Slot { started_at: u64 },
    Epoch { started_at: u64 },
    Timestamp { started_at: i64 },
    Pyth { price: i64 },
    Periodic { started_at: i64 },
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, Hash, PartialEq)]
pub struct SerializableInstruction {
    pub program_id: Pubkey,
    pub accounts: Vec<SerializableAccount>,
    pub data: Vec<u8>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, Hash, PartialEq)]
pub struct SerializableAccount {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}
