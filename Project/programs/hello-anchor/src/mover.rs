use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token};

#[program]
pub mod enviador_aqua {
    use super::*;

    pub fn enviar_aqua(ctx: Context<EnviarTokens>) -> Result<()> {
        // 1. Definimos quién envía, quién recibe y quién autoriza (tú)
        let cpi_accounts = Transfer {
            from: ctx.accounts.mi_cuenta_aqua.to_account_info(),
            to: ctx.accounts.cuenta_destino_aqua.to_account_info(),
            authority: ctx.accounts.mi_wallet.to_account_info(),
        };

        // 2. Preparamos el contexto del programa oficial de Tokens de Solana
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // 3. Calculamos la cantidad (200 tokens con 6 decimales)
        // 200 * 1,000,000 = 200,000,000
        let cantidad: u64 = 200 * 10u64.pow(6);

        // 4. ¡Hacemos el envío!
        token::transfer(cpi_ctx, cantidad)?;

        msg!("¡Enviados 200 AQUA a la wallet Bdohpv...!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EnviarTokens<'info> {
    #[account(mut)]
    pub mi_cuenta_aqua: Account<'info, anchor_spl::token::TokenAccount>, // Mi almacén de AQUA
    #[account(mut)]
    pub cuenta_destino_aqua: Account<'info, anchor_spl::token::TokenAccount>, // El almacén de AQUA de la otra persona
    pub mi_wallet: Signer<'info>, // Yo, que firmo la transacción
    pub token_program: Program<'info, Token>, // El "notario" (Programa de Tokens)
}