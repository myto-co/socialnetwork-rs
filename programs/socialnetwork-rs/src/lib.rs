use anchor_lang::prelude::*;
use anchor_lang::system_program;
use instructions::*;

pub mod errors;
pub mod state;
pub mod instructions;

declare_id!("HJ52iVx4M1A29841eXbT2SCUo1Gfcho7AzFPjGH24sys");

#[program]
pub mod socialnetwork_rs {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;

        // Transfer SOL for PDA rent
        let rent_xfer_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(), 
            system_program::Transfer{
                from: ctx.accounts.initializer.to_account_info(),
                to: ctx.accounts.sol_vault.to_account_info()
            }
        );
        let rent_xfer_res = system_program::transfer(rent_xfer_context, 20000000);
        if !rent_xfer_res.is_ok() {
            return err!(errors::TiktaalikError::SolTransferFailed);
        }

        state.state_bump = ctx.bumps.state;
        state.vault_bump = ctx.bumps.sol_vault;
        Ok(())
    }

    pub fn create_post(ctx: Context<CreatePost>, content_hash: String) -> Result<()> {
        require!(content_hash.len() > 0, errors::TiktaalikError::EmptyContent);
        let post = &mut ctx.accounts.post;
        post.content_hash = content_hash;
        post.time_created = Clock::get()?.unix_timestamp.try_into().unwrap();
        post.creator = ctx.accounts.authority.key();

        Ok(())
    }

    pub fn register_user(ctx: Context<RegisterUser>, username: String) -> Result<()> {
        let user = &mut ctx.accounts.user;
        let name_registry = &mut ctx.accounts.name_registry;

        user.authority = ctx.accounts.authority.key();
        user.username = username;
        user.bump = ctx.bumps.user;
        name_registry.user = ctx.accounts.authority.key();
        name_registry.bump = ctx.bumps.name_registry;

        Ok(())
    }

    pub fn seed_post(ctx: Context<SeedPost>, amount: u64) -> Result<()> {
        let post = &mut ctx.accounts.post;
        let seeder = &mut ctx.accounts.seeder;

        // Init seeder info if needed.
        if !seeder.is_init {
            seeder.authority = ctx.accounts.payer.key();
            seeder.is_init = true;
            seeder.bump = ctx.bumps.seeder;
        }

        // Calculate split for poster and seeder.
        let poster_cut = (amount * 25) / 100;
        let seeder_cut = amount - poster_cut;

        // Transfer in SOL from the seeder to the poster.
        let poster_xfer_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.poster.to_account_info()
            }
        );
        let poster_xfer_res = system_program::transfer(poster_xfer_context, poster_cut);
        if !poster_xfer_res.is_ok() {
            return err!(errors::TiktaalikError::SolTransferFailed);
        }

        // Transfer remaining `amount` to the program.
        let seed_xfer_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.vault.to_account_info()
            }
        );
        let seed_xfer_res = system_program::transfer(seed_xfer_context, seeder_cut);
        if !seed_xfer_res.is_ok() {
            return err!(errors::TiktaalikError::SolTransferFailed);
        }

        // Update seeds with new amount.
        let _ = post.update_seeds(seeder_cut).is_ok();

        // Update any earned rewards for the seeder.
        seeder.seed_rewards = ((seeder.seed_shares * (post.reward_per_seed - seeder.seed_paid_out_per_shares)) / (10^9)) + seeder.seed_rewards;
        seeder.seed_paid_out_per_shares = post.reward_per_seed;

        // Add shares to the seeder.
        post.total_seed_shares += seeder_cut;
        seeder.seed_shares += seeder_cut;

        Ok(())
    }

    pub fn claim_seed_rewards(ctx: Context<ClaimSeed>) -> Result<()> {
        let post = &mut ctx.accounts.post;
        let seeder = &mut ctx.accounts.seeder;

        // Update any earned rewards for the seeder.
        seeder.seed_rewards = ((seeder.seed_shares * (post.reward_per_seed - seeder.seed_paid_out_per_shares)) / (10^9)) + seeder.seed_rewards;
        seeder.seed_paid_out_per_shares = post.reward_per_seed;

        // Send rewards.
        let reward = seeder.seed_rewards;
        if reward > 0 {
            seeder.seed_rewards = 0;
            ctx.accounts.vault.sub_lamports(reward)?;
            ctx.accounts.authority.add_lamports(reward)?;
        }

        Ok(())
    }

    pub fn create_comment(ctx: Context<CreateComment>, content_hash: String) -> Result<()> {
        require!(content_hash.len() > 0, errors::TiktaalikError::EmptyContent);
        let comment = &mut ctx.accounts.comment;
        let post = &mut ctx.accounts.post;

        comment.content_hash = content_hash;
        comment.creator = ctx.accounts.payer.key();
        comment.post = post.key();
        comment.bump = ctx.bumps.comment;
        post.comments_count += 1;

        Ok(())
    }

    pub fn update_profile(ctx: Context<UpdateProfile>, new_hash: String) -> Result<()> {
        require!(new_hash.len() > 0, errors::TiktaalikError::EmptyContent);
        ctx.accounts.profile.metadata_hash = new_hash;
        Ok(())
    }

    pub fn change_username(ctx: Context<ChangeUsername>, username: String) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        let new_registry = &mut ctx.accounts.new_registry;

        new_registry.user = profile.key();
        new_registry.bump = ctx.bumps.new_registry;
        profile.username = username;

        Ok(())
    }

    pub fn edit_post(ctx: Context<EditPost>, new_hash: String) -> Result<()> {
        require!(new_hash.len() > 0, errors::TiktaalikError::EmptyContent);
        ctx.accounts.post.content_hash = new_hash;
        Ok(())
    }
}