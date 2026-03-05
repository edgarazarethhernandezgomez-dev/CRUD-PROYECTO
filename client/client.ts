async function gestionarTickets() {
  // Generamos un ID único para este ticket (u64)
  const ticketId = new anchor.BN(Math.floor(Math.random() * 10000));
  
  // Derivar la PDA usando el ID
  const [ticketPDA] = await anchor.web3.PublicKey.findProgramAddress(
    [
      Buffer.from("ticket"),
      pg.wallet.publicKey.toBuffer(),
      ticketId.toArrayLike(Buffer, "le", 8)
    ],
    pg.program.programId
  );

  console.log(`🎫 Ticket ID: ${ticketId.toString()}`);
  console.log(`📍 PDA del Ticket: ${ticketPDA.toBase58()}`);

  // 1. ABRIR TICKET
  await pg.program.methods
    .abrirTicket(ticketId, "Error en Login", "No puedo entrar a mi cuenta")
    .accounts({
      autor: pg.wallet.publicKey,
      ticket: ticketPDA,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
  
  console.log("✅ Ticket creado en la blockchain.");

  // 2. ACTUALIZAR ESTADO (A 'EnProceso')
  await pg.program.methods
    .actualizarEstado({ enProceso: {} })
    .accounts({
      autor: pg.wallet.publicKey,
      ticket: ticketPDA,
    })
    .rpc();
  
  console.log("⚙️ Estado actualizado a 'En Proceso'.");

  // 3. LEER DATOS
  const ticketData = await pg.program.account.ticket.fetch(ticketPDA);
  console.log("--- DATOS DEL TICKET ---");
  console.log(`Asunto: ${ticketData.asunto}`);
  console.log(`Estado: ${Object.keys(ticketData.estado)[0]}`);

  // 4. CERRAR TICKET (Opcional - Elimina la cuenta y te devuelve el SOL)
  /*
  await pg.program.methods
    .cerrarTicket()
    .accounts({
      autor: pg.wallet.publicKey,
      ticket: ticketPDA,
    })
    .rpc();
  console.log("🗑️ Ticket eliminado y renta recuperada.");
  */
}

gestionarTickets();
