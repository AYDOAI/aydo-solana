import {
    getAssociatedTokenAddressSync,
    createAssociatedTokenAccountInstruction,
    TOKEN_PROGRAM_ID,
    TOKEN_2022_PROGRAM_ID
} from "@solana/spl-token";

describe("Buyer", () => {
    it("deposit_funds", async () => {
        const provider = anchor.AnchorProvider.env();
        anchor.setProvider(provider);

        const wallet = provider.wallet as anchor.Wallet;
        const payer = wallet.publicKey; // ✅ Используем wallet.publicKey
        const programId = pg.program.programId;
        const mint = new web3.PublicKey("AYMuaTVib2XrPwStrWaVW7k2yeZmDVRc61SFvDjJTwF3");
        const tokenProgramId = TOKEN_2022_PROGRAM_ID;

        console.log("Payer:", payer.toBase58());

        const [buyer, bump] = web3.PublicKey.findProgramAddressSync(
            [Buffer.from("buyer"), payer.toBuffer()],
            programId
        );

        console.log(`Using mint: ${mint.toBase58()}`);
        console.log(`Using token program id: ${tokenProgramId}`);

        const senderTokenAccount = getAssociatedTokenAddressSync(mint, payer, true, tokenProgramId);
        console.log("Sender Token Account:", senderTokenAccount.toBase58());

        const senderBalance = await pg.connection.getTokenAccountBalance(senderTokenAccount);
        console.log(`Sender Token Balance: ${senderBalance.value.amount}`);

        const buyerTokenAccount = getAssociatedTokenAddressSync(mint, buyer, true, tokenProgramId);
        console.log("Buyer Token Account:", buyerTokenAccount?.toBase58());

        let buyerTokenAccountInfo = await pg.connection.getAccountInfo(buyerTokenAccount);
        if (!buyerTokenAccountInfo) {
            console.log("Creating buyer's token account...");

            const createATAIx = createAssociatedTokenAccountInstruction(
                payer,
                buyerTokenAccount,
                buyer,
                mint,
                tokenProgramId
            );

            const createTx = new web3.Transaction().add(createATAIx);
            await web3.sendAndConfirmTransaction(pg.connection, createTx, [wallet.payer]);

            buyerTokenAccountInfo = await pg.connection.getAccountInfo(buyerTokenAccount);
            if (!buyerTokenAccountInfo) {
                throw new Error("buyerTokenAccount was not created successfully!");
            }
        }

        const buyerBalance = await pg.connection.getTokenAccountBalance(buyerTokenAccount);
        console.log(`Buyer Token Balance: ${buyerBalance.value.amount}`);

        const amount = new anchor.BN(1000);

        const txHash = await pg.program.methods
            .depositFunds(amount)
            .accounts({
                buyer: buyer,
                owner: payer,
                ownerTokenAccount: senderTokenAccount,
                buyerTokenAccount: buyerTokenAccount,
                tokenProgram: tokenProgramId,
                systemProgram: web3.SystemProgram.programId,
            })
            .rpc();

        console.log(`Transaction sent: ${txHash}`);
    });
});