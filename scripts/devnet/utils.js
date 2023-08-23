const web3 = require('@solana/web3.js');

const connection = new web3.Connection('https://api.devnet.solana.com', {
  commitment: 'confirmed',
});

exports.processInstruction = async function (instruction, payer) {
  const transaction = new web3.Transaction();
  transaction.add(instruction);
  /*
  transaction.add(
    ComputeBudgetProgram.setComputeUnitLimit({
      units: 400_000,
    })
  );
  */
  transaction.feePayer = payer.publicKey;

  const result = await web3.sendAndConfirmTransaction(
    connection,
    transaction,
    [payer],
    {
      commitment: 'confirmed',
    }
  );
  console.log('result', result);
};
