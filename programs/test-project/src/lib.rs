use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction::transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// https://book.anchor-lang.com/anchor_references/space.html.
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const BIT_8_LENGTH: usize = 1;
const BIT_64_LENGTH: usize = 8;
const FLOAT_64_LENGTH: usize = 8;
const ENUM_LENGTH: usize = 1 + 1;

#[program]
pub mod test_project {
    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>,
        default_time_increase_per_bet: i64,
        minimal_time_increase_per_bet: i64,
        auction_duration: i64,
        max_participation_amount: u64,
        min_pot_size: u64,
    ) -> Result<()> {
        let state_account: &mut Account<State> = &mut ctx.accounts.auction_instance;
        let vault: &mut Account<Vault> = &mut ctx.accounts.vault;

        state_account.vault_public_key = ctx.accounts.vault.key().clone();

        state_account.auction_state = AuctionState::ENDED;

        state_account.default_time_increase_per_bet = default_time_increase_per_bet;
        state_account.minimal_time_increase_per_bet = minimal_time_increase_per_bet;
        state_account.auction_duration = auction_duration;
        state_account.max_participation_amount = max_participation_amount;
        state_account.min_pot_size = min_pot_size;

        state_account.time_decrease_per_bet_reduction_speed = 1;

        state_account.treasury_to_pot_size_percentage = 0.1;
        state_account.pot_size_to_participation_amount_percentage = 0.01;
        state_account.pot_size_fee_percentage = 0.09;

        Ok(())
    }

    pub fn fund_vault(ctx: Context<FundVault>, amount: u64) -> Result<()> {
        let payer = &mut ctx.accounts.payer;
        let receiver = &mut ctx.accounts.vault;

        if payer.lamports() < amount {
            return Err(error!(ErrorCode::NotEnoughSol));
        }

        let instruction = transfer(&payer.key(), &receiver.key(), amount);

        let account_infos = [
            payer.to_account_info().clone(),
            receiver.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ];

        invoke(&instruction, &account_infos)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, seeds = [b"vault"], bump, space = Vault::LEN)]
    pub vault: Account<'info, Vault>,
    #[account(init, payer = user, seeds = [b"state"], bump, space = State::LEN)]
    pub auction_instance: Account<'info, State>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FundVault<'info> {
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct State {
    default_time_increase_per_bet: i64,
    minimal_time_increase_per_bet: i64,
    auction_duration: i64,
    max_participation_amount: u64,
    min_pot_size: u64,
    auction_state: AuctionState,
    time_decrease_per_bet_reduction_speed: u64,
    // ongoing auction, to own account?
    current_auction_start_time: i64,
    current_auction_starting_pot_size: u64,
    current_auction_number: u64,
    current_participation_amount: u64,
    // fees
    treasury_to_pot_size_percentage: f64,
    pot_size_to_participation_amount_percentage: f64,
    pot_size_fee_percentage: f64,
    // current winner
    current_winning_bid_made_at: i64,
    current_winning_pubkey: Pubkey,
    //
    vault_public_key: Pubkey,
}

#[account]
pub struct Vault {
    bump: u8,
}

impl Vault {
    const LEN: usize = DISCRIMINATOR_LENGTH + BIT_8_LENGTH;
}

// 3. Add a constant on the Tweet account that provides its total size.
impl State {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + 5 * TIMESTAMP_LENGTH
        + 6 * BIT_64_LENGTH
        + 3 * FLOAT_64_LENGTH
        + ENUM_LENGTH
        + 2 * PUBLIC_KEY_LENGTH;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
enum AuctionState {
    STARTED,
    ENDED,
    LOTTERY,
    WAITING,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Not enough SOL")]
    NotEnoughSol,
}
