use anchor_lang::prelude::*;

declare_id!("GzX76Vk8XLLJxYkpAAwaeu94DuJXTzLQ8zkog4xp34vd");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    // Get a reference to the account.
    let base_account = &mut ctx.accounts.base_account;
    // Initialize total_gifs.
    base_account.total_gifs = 0;
    Ok(())
  }

  // The function now accepts a gif_link param from the user. We also reference the user from the Context
  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

	// Build the struct.
    let item = ItemStruct {
      gif_link: gif_link.to_string(),
      user_address: *user.to_account_info().key,
      upvotes: 0,
    };
		
	// Add it to the gif_list vector.
    base_account.gif_list.push(item);
    base_account.total_gifs += 1;
    Ok(())
  }

  pub fn upvote_gif(ctx: Context<UpvoteGif>, gif_link: String) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    for gif in &mut base_account.gif_list {
        if gif.gif_link == gif_link {
            gif.upvotes += 1;
        }
    }
    Ok(())
  }

  pub fn tip_small_sol(ctx: Context<TipSmallSol>) -> ProgramResult {
    let tx = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.from.key(),
        &ctx.accounts.to.key(),
        10000000
    );
    anchor_lang::solana_program::program::invoke(
        &tx,
        &[
            ctx.accounts.from.to_account_info(),
            ctx.accounts.to.to_account_info(),
        ],
    );
    Ok(())
  }
}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Add the signer who calls the AddGif method to the struct so that we can save it
#[derive(Accounts)]
pub struct AddGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpvoteGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct TipSmallSol<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  from: Signer<'info>,
  #[account(mut)]
  to: AccountInfo<'info>,
  system_program: Program<'info, System>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub upvotes: u64,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
	// Attach a Vector of type ItemStruct to the account.
    pub gif_list: Vec<ItemStruct>,
}