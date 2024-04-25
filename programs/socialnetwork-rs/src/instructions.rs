use crate::state::*;
use crate::errors::*;
use anchor_lang::prelude::*;
use zerocopy::AsBytes;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(
        init,
        space = 8 + AppState::MAX_SIZE,
        payer = initializer,
        seeds = [b"tiktaalik-state"],
        bump
    )]
    pub state: Account<'info, AppState>,
    #[account(
        mut,
        seeds = [b"tiktaalik-vault"],
        bump
    )]
    pub sol_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        space = 8 + Post::MAX_SIZE,
        payer = authority
    )]
    pub post: Account<'info, Post>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct EditPost<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut, has_one = creator @ TiktaalikError::AuthorityDoesNotMatch)]
    pub post: Account<'info, Post>
}

#[derive(Accounts)]
#[instruction(username: String)]
pub struct RegisterUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + User::MAX_SIZE,
        seeds = [authority.key().as_ref()],
        bump
    )]
    pub user: Account<'info, User>,
    #[account(
        init,
        payer = authority,
        space = 8 + NameRegistry::MAX_SIZE,
        seeds = [username.as_bytes().as_ref()],
        bump
    )]
    pub name_registry: Account<'info, NameRegistry>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(username: String)]
pub struct ChangeUsername<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub destination: SystemAccount<'info>,
    #[account(mut, has_one = authority @ TiktaalikError::AuthorityDoesNotMatch)]
    pub profile: Account<'info, User>,
    #[account(
        mut,
        seeds = [profile.username.as_bytes().as_ref()],
        bump = prev_registry.bump,
        close = destination
    )]
    pub prev_registry: Account<'info, NameRegistry>,
    #[account(
        init,
        payer = authority,
        space = 8 + NameRegistry::MAX_SIZE,
        seeds = [username.as_bytes().as_ref()],
        bump
    )]
    pub new_registry: Account<'info, NameRegistry>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    pub authority: Signer<'info>,
    #[account(mut, has_one = authority @ TiktaalikError::AuthorityDoesNotMatch)]
    pub profile: Account<'info, User>
}

#[derive(Accounts)]
pub struct SeedPost<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub post: Account<'info, Post>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + Seeder::MAX_SIZE,
        seeds = [payer.key().as_ref(), post.key().as_ref()],
        bump
    )]
    pub seeder: Account<'info, Seeder>,
    #[account(
        seeds = [b"tiktaalik-state"],
        bump = state.state_bump
    )]
    pub state: Account<'info, AppState>,
    #[account(
        mut,
        seeds = [b"tiktaalik-vault"],
        bump = state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(mut, constraint = poster.key() == post.creator @ TiktaalikError::InvalidPoster)]
    /// CHECK: This account won't be read or written to.
    pub poster: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct ClaimSeed<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub post: Account<'info, Post>,
    #[account(
        mut,
        seeds = [authority.key().as_ref(), post.key().as_ref()],
        bump = seeder.bump,
        has_one = authority @ TiktaalikError::AuthorityDoesNotMatch
    )]
    pub seeder: Account<'info, Seeder>,
    #[account(
        seeds = [b"tiktaalik-state"],
        bump = state.state_bump
    )]
    pub state: Account<'info, AppState>,
    #[account(
        mut,
        seeds = [b"tiktaalik-vault"],
        bump = state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct CreateComment<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub post: Account<'info, Post>,
    #[account(
        init,
        payer = payer,
        space = 8 + Comment::MAX_SIZE,
        seeds = [payer.key().as_ref(), post.key().as_ref(), post.comments_count.as_bytes().as_ref()],
        bump
    )]
    pub comment: Account<'info, Comment>,
    pub system_program: Program<'info, System>
}