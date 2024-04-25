use anchor_lang::prelude::*;

#[account]
pub struct AppState {
    pub vault: Pubkey,
    pub state_bump: u8, // 1
    pub vault_bump: u8 // 1
}

impl AppState {
    pub const MAX_SIZE: usize = 32 + 1 + 1;
}

#[account]
pub struct User {
    pub authority: Pubkey, // 32
    pub username: String, // 16
    pub metadata_hash: String, // 64
    pub bump: u8 // 1
}

impl User {
    pub const MAX_SIZE: usize = 32 + 16 + 64 + 1;
}

#[account]
pub struct NameRegistry {
    pub user: Pubkey, // 32
    pub bump: u8 // 1
}

impl NameRegistry {
    pub const MAX_SIZE: usize = 32 + 1;
}

#[account]
pub struct Seeder {
    pub authority: Pubkey, // 32
    pub seed_shares: u64, // 8
    pub seed_paid_out_per_shares: u64, // 8
    pub seed_rewards: u64, // 8
    pub is_init: bool, // 1
    pub bump: u8 // 1
}

impl Seeder {
    pub const MAX_SIZE: usize = 32 + 8 + 8 + 8 + 1 + 1;
}

#[account]
pub struct Post {
    pub creator: Pubkey, // 32
    pub content_hash: String, // 64, enough room for a proper hash
    pub time_created: u64, // 8
    pub total_seed_shares: u64, // 8
    pub reward_per_seed: u64, // 8
    pub comments_count: u16 // 2
}

impl Post {
    pub const MAX_SIZE: usize = 32 + 64 + 8 + 8 + 8 + 2;

    pub fn update_seeds(&mut self, amount: u64) -> Result<()> {
        if self.total_seed_shares == 0 {
            return Ok(())
        }

        self.reward_per_seed = self.reward_per_seed + ((amount * 10^9) / self.total_seed_shares);
        Ok(())
    }
}

#[account]
pub struct Comment {
    pub creator: Pubkey, // 32
    pub post: Pubkey, // 32
    pub content_hash: String, // 64
    pub time_created: u64, // 8
    pub bump: u8 // 1
}

impl Comment {
    pub const MAX_SIZE: usize = 32 + 64 + 8 + 1;
}