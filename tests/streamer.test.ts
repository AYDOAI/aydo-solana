import {
    getAssociatedTokenAddressSync,
    createAssociatedTokenAccountInstruction,
    TOKEN_PROGRAM_ID,
    TOKEN_2022_PROGRAM_ID
} from "@solana/spl-token";

describe("Streamer", () => {
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
        const [streamer, bump] = web3.PublicKey.findProgramAddressSync(
            [Buffer.from("streamer"), owner.toBuffer()],
            programId
        );

        console.log(`Streamer: ${streamer}`);

        const streamerTokenAccount = getAssociatedTokenAddressSync(mint, streamer, true, tokenProgramId);
        console.log("Streamer Token Account:", streamerTokenAccount?.toBase58());

        let streamerTokenAccountInfo = await pg.connection.getAccountInfo(streamerTokenAccount);
        if (!streamerTokenAccountInfo) {
            console.log("Creating streamer's token account...");

            const createATAIx = createAssociatedTokenAccountInstruction(
                owner,
                streamerTokenAccount,
                streamer,
                mint,
                tokenProgramId
            );

            const createTx = new web3.Transaction().add(createATAIx);
            await web3.sendAndConfirmTransaction(pg.connection, createTx, [wallet.payer]);

            streamerTokenAccountInfo = await pg.connection.getAccountInfo(streamerTokenAccount);
            if (!streamerTokenAccountInfo) {
                throw new Error("Streamer Token Account was not created successfully!");
            }
        }

        const streamerBalance = await pg.connection.getTokenAccountBalance(streamerTokenAccount);
        console.log(`Streamer Token Balance: ${streamerBalance.value.amount}`);

        const txHash = await pg.program.methods
            .createStreamer()
            .accounts({
                streamer: streamer,
                owner: owner,
                systemProgram: web3.SystemProgram.programId,
            })
            .rpc();
        console.log(`Transaction sent: ${txHash}`);

        const streamerData = await pg.program.account.streamer.fetch(
            streamer
        );

        console.log(`Streamer Data:`);
        console.log(streamerData);
    });
});