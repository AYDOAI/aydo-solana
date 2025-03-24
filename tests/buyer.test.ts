import {
    getAssociatedTokenAddressSync,
    createAssociatedTokenAccountInstruction,
    TOKEN_PROGRAM_ID,
    TOKEN_2022_PROGRAM_ID
} from "@solana/spl-token";

describe("Buyer", async () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const wallet = provider.wallet as anchor.Wallet;
    const owner = wallet.publicKey;
    const programId = pg.program.programId;
    const mint = new web3.PublicKey("AYMuaTVib2XrPwStrWaVW7k2yeZmDVRc61SFvDjJTwF3");
    const tokenProgramId = TOKEN_2022_PROGRAM_ID;

    console.log("Owner:", owner.toBase58());
    console.log(`Using mint: ${mint.toBase58()}`);
    console.log(`Using token program id: ${tokenProgramId}`);

    it("Create", async () => {
        const [buyer, bump] = web3.PublicKey.findProgramAddressSync(
            [Buffer.from("buyer"), owner.toBuffer()],
            programId
        );

        console.log(`Buyer: ${buyer}`);

        const buyerTokenAccount = getAssociatedTokenAddressSync(mint, buyer, true, tokenProgramId);
        console.log("Buyer Token Account:", buyerTokenAccount?.toBase58());

        let buyerTokenAccountInfo = await pg.connection.getAccountInfo(buyerTokenAccount);
        if (!buyerTokenAccountInfo) {
            console.log("Creating buyer's token account...");

            const createATAIx = createAssociatedTokenAccountInstruction(
                owner,
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

        const txHash = await pg.program.methods
            .createBuyer()
            .accounts({
                buyer: buyer,
                owner: owner,
                systemProgram: web3.SystemProgram.programId,
            })
            .rpc();
        console.log(`Transaction sent: ${txHash}`);

        const buyerData = await pg.program.account.buyer.fetch(
            buyer
        );

        console.log(`Buyer Data:`);
        console.log(buyerData);
    });

    it("Deposit Funds", async () => {
        const [buyer, bump] = web3.PublicKey.findProgramAddressSync(
            [Buffer.from("buyer"), owner.toBuffer()],
            programId
        );

        console.log(`Buyer: ${buyer}`);

        const buyerTokenAccount = getAssociatedTokenAddressSync(mint, buyer, true, tokenProgramId);
        console.log("Buyer Token Account:", buyerTokenAccount?.toBase58());

        let buyerTokenAccountInfo = await pg.connection.getAccountInfo(buyerTokenAccount);
        if (!buyerTokenAccountInfo) {
            console.log("Creating buyer's token account...");

            const createATAIx = createAssociatedTokenAccountInstruction(
                owner,
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

        const senderTokenAccount = getAssociatedTokenAddressSync(mint, owner, true, tokenProgramId);
        console.log("Sender Token Account:", senderTokenAccount.toBase58());

        const senderBalance = await pg.connection.getTokenAccountBalance(senderTokenAccount);
        console.log(`Sender Token Balance: ${senderBalance.value.amount}`);

        const amount = new anchor.BN(10000);

        const txHash = await pg.program.methods
            .depositFunds(amount)
            .accounts({
                buyer: buyer,
                owner: owner,
                ownerTokenAccount: senderTokenAccount,
                buyerTokenAccount: buyerTokenAccount,
                mint: mint,
                tokenProgram: tokenProgramId,
                systemProgram: web3.SystemProgram.programId,
            })
            .rpc();

        console.log(`Transaction sent: ${txHash}`);

        const buyerData = await pg.program.account.buyer.fetch(
            buyer
        );

        console.log(`Buyer Data:`);
        console.log(buyerData);
    });
});