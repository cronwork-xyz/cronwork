use anchor_lang::prelude::*;

use crate::constant::SEED_THREAD;

pub fn get_thread_pda(authority: Pubkey, id: Vec<u8>, domain: Option<Vec<u8>>) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            SEED_THREAD,
            authority.as_ref(),
            id.as_slice(),
            domain.unwrap_or_default().as_slice(),
        ],
        &crate::ID,
    )
}
