use anchor_lang::prelude::*;

#[event]
pub struct PostEdited {
}

#[event]
pub struct UserUpdated {

}

#[event]
pub struct UsernameChanged {

}

#[event]

pub struct PostSeeded {
    post: Pubkey,
    seeder: Pubkey,
    amount: u64
}