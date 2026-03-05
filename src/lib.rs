use anchor_lang::prelude::*;

declare_id!("6rpxjPrriwCknspBnhrA4mBgbBqm4Nnpv6E6i5ZfDr8F");

#[program]
pub mod sistema_tickets {
    use super::*;

    // Crea un ticket individual usando un ID como semilla
    pub fn abrir_ticket(ctx: Context<CrearTicket>, id: u64, asunto: String, descripcion: String) -> Result<()> {
        let ticket = &mut ctx.accounts.ticket;
        ticket.autor = ctx.accounts.autor.key();
        ticket.id = id;
        ticket.asunto = asunto;
        ticket.descripcion = descripcion;
        ticket.estado = EstadoTicket::Abierto;
        
        msg!("Ticket #{} abierto correctamente", id);
        Ok(())
    }

    // Actualiza el estado del ticket
    pub fn actualizar_estado(ctx: Context<GestionarTicket>, nuevo_estado: EstadoTicket) -> Result<()> {
        let ticket = &mut ctx.accounts.ticket;
        ticket.estado = nuevo_estado;
        msg!("Estado del ticket #{} actualizado", ticket.id);
        Ok(())
    }

    // Elimina el ticket (Cierra la cuenta y recupera el SOL de la renta)
    pub fn cerrar_ticket(_ctx: Context<GestionarTicket>) -> Result<()> {
        msg!("Ticket cerrado y cuenta eliminada.");
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace, Debug)]
pub enum EstadoTicket {
    Abierto,
    EnProceso,
    Resuelto,
}



#[account]
#[derive(InitSpace)]
pub struct Ticket {
    pub autor: Pubkey,
    pub id: u64,
    #[max_len(50)]
    pub asunto: String,
    #[max_len(200)]
    pub descripcion: String,
    pub estado: EstadoTicket,
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CrearTicket<'info> {
    #[account(mut)]
    pub autor: Signer<'info>,

    #[account(
        init,
        payer = autor,
        space = 8 + Ticket::INIT_SPACE,
        // Usamos el ID y el autor como semillas para que sean únicos
        seeds = [b"ticket", autor.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub ticket: Account<'info, Ticket>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionarTicket<'info> {
    #[account(mut)]
    pub autor: Signer<'info>,

    #[account(
        mut,
        close = autor, // Al eliminar, el SOL regresa al autor
        seeds = [b"ticket", autor.key().as_ref(), ticket.id.to_le_bytes().as_ref()],
        bump,
        has_one = autor
    )]
    pub ticket: Account<'info, Ticket>,
}
