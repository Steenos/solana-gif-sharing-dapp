const anchor = require('@project-serum/anchor');

//need the system program
const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log('🚀  starting test...')

  //create and set the provider. we set it before but neee to update it.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Mysolproject;
  
  //create and account keypair for our program to use.
  const baseAccount = anchor.web3.Keypair.generate();

  //call start_stuff_off, pass it the params it needs
  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });
  

  console.log("📝  Your transaction sig: ", tx);

  //fetch data from account.
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Gif Count: ', account.totalGifs.toString())

   //Call add_gif!
   await program.rpc.addGif("https://media.giphy.com/media/tqtH0MJyxJiwwW7aEy/giphy.gif",{
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  //Get the account again to see what changed.
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀  Gif count: ', account.totalGifs.toString());

  //access the gif_list
  console.log('👀  Gif List: ', account.gifList)
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();