import * as StellarSDK from "@stellar/stellar-sdk";
import dotenv from "dotenv";

dotenv.config();

if (!process.env.BASIC_COUNTER_CONTRACT_ADDRESS || !process.env.SECRET_KEY) {
  throw new Error("Missing environment variable");
}

const server = new StellarSDK.rpc.Server("https://soroban-testnet.stellar.org");

const contractId = process.env.BASIC_COUNTER_CONTRACT_ADDRESS;

const sourceKeypair = StellarSDK.Keypair.fromSecret(process.env.SECRET_KEY);

async function increment() {
  try {
    const account = await server.getAccount(sourceKeypair.publicKey());

    const contract = new StellarSDK.Contract(contractId);

    const tx = new StellarSDK.TransactionBuilder(account, {
      fee: StellarSDK.BASE_FEE,
      networkPassphrase: StellarSDK.Networks.TESTNET,
    })
      .addOperation(contract.call("increment"))
      .setTimeout(30)
      .build();

    const preparedTx = await server.prepareTransaction(tx);

    preparedTx.sign(sourceKeypair);

    const response = await server.sendTransaction(preparedTx);

    let result = await server.getTransaction(response.hash);

    let attempts = 0;

    while (result.status === "NOT_FOUND" && attempts < 20) {
      console.log("Waiting ...");
      await new Promise((r) => setTimeout(r, 1000));
      result = await server.getTransaction(response.hash);
      attempts++;
    }

    console.log("Final status:", result.status);

    if (result.status === "SUCCESS") {
      console.log("RAW RESULT:", result.returnValue);
      console.log("VALUE:", StellarSDK.scValToNative(result.returnValue!));
    } else {
      console.log("FAILED:", result);
    }
  } catch (e) {
    console.error("ERROR:", e);
  }
}

(async () => {
  try {
    await increment();
  } catch (e) {
    console.error("Fatal error:", e);
  }
})();
