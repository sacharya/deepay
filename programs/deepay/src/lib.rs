use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod deepay {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }

    pub fn create_loan(ctx: Context<CreateLoan>) -> ProgramResult{
        let loan = &mut ctx.accounts.loan;
        loan.active = true;
        Ok(())
    }

    pub fn cancel_loan(ctx: Context<CancelLoan>) -> ProgramResult{
        let loan = &mut ctx.accounts.loan;
        loan.active = false;
        Ok(())
    }

    pub fn bid(ctx: Context<Bid>) -> ProgramResult{
        let _ = &mut ctx.accounts.loan;
        Ok(())
    }

    pub fn underwrite_loan(ctx: Context<UnderWriteLoan>) -> ProgramResult{
        let _ = &mut ctx.accounts.loan;
        Ok(())
    }

    pub fn draw_loan(ctx: Context<DrawLoan>) -> ProgramResult{
        let _ = &mut ctx.accounts.loan;
        Ok(())
    }

    pub fn calculate_payment(ctx: Context<CalculatePayment>) -> ProgramResult{
        let _ = &mut ctx.accounts.loan;
        Ok(())
    }

    pub fn default_loan(ctx: Context<DefaultLoan>) -> ProgramResult{
        let _ = &mut ctx.accounts.loan;
        Ok(())
    }

    pub fn pay_loan(ctx: Context<PayLoan>) -> ProgramResult{
        let _ = &mut ctx.accounts.loan;
        Ok(())
    }

    pub fn seize_collateral(ctx: Context<SeizeCollateral>) -> ProgramResult{
        let _ = &mut ctx.accounts.loan;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct CreateLoan<'info> {
    #[account(mut, "loan.active")]
    loan: ProgramAccount<'info, Loan>,
}

#[derive(Accounts)]
pub struct CancelLoan<'info> {
    #[account(mut, "loan.active")]
    loan: ProgramAccount<'info, Loan>,
}

#[derive(Accounts)]
pub struct Bid<'info> {
    #[account(mut, "loan.active")]
    loan: ProgramAccount<'info, Loan>,
}

#[derive(Accounts)]
pub struct UnderWriteLoan<'info> {
    #[account(mut, "loan.active")]
    loan: ProgramAccount<'info, Loan>,
}

#[derive(Accounts)]
pub struct DrawLoan<'info> {
    #[account(mut, "loan.active")]
    loan: ProgramAccount<'info, Loan>,
}

#[derive(Accounts)]
pub struct CalculatePayment<'info> {
    #[account(mut, "loan.active")]
    loan: ProgramAccount<'info, Loan>,
}

#[derive(Accounts)]
pub struct DefaultLoan<'info> {
    #[account(mut, "loan.active")]
    loan: ProgramAccount<'info, Loan>,
}

#[derive(Accounts)]
pub struct PayLoan<'info> {
    #[account(mut, "loan.active")]
    loan: ProgramAccount<'info, Loan>,
}

#[derive(Accounts)]
pub struct SeizeCollateral<'info> {
    #[account(mut, "loan.active")]
    loan: ProgramAccount<'info, Loan>,
}

#[account]
pub struct Loan {
    active: bool,
    borrower: Pubkey,
    lender: Pubkey,
}

