// import { Keypair } from "@solana/web3.js";
const { Keypair, Connection, PublicKey, LAMPORTS_PER_SOL } = require("@solana/web3.js");

const privateKeyArray = [25, 250, 154, 253, 170, 251, 132, 37, 46, 128,   
    46, 49, 89, 8, 156, 186, 229, 154, 111,   
    26, 196, 50, 198, 218, 89, 140, 14, 199,   
    215, 55, 222, 164, 8, 234, 188, 216,   
    71, 250, 244, 237, 252, 154, 218, 63,   
    68, 81, 162, 103, 53, 14, 61, 180,   
    243, 70, 69, 157, 107, 136, 133, 38,   
    209, 239, 66, 247];

function main() {
    // Convert the array to Uint8Array  
    const privateKey = Uint8Array.from(privateKeyArray);  

    // Create Keypair from secret key  
    const keypair = Keypair.fromSecretKey(privateKey);  

    // Convert private key to base58  
    const privateKeyBase58 = Buffer.from(keypair.secretKey).toString('base64');  

    // Convert to public key
    const pubKey = keypair.publicKey.toBase58();

    // Output the public key and private key in base58 format  
    console.log("Public Key:", pubKey);  
    console.log("Private Key (Base58):", privateKeyBase58);

    requestAirdrop(pubKey);
}

async function requestAirdrop(publicKeyString) {
    const connection = new Connection("https://api.devnet.solana.com", "confirmed");
    const publicKey = new PublicKey(publicKeyString);
    const AIRDROP_AMOUNT = 3 * LAMPORTS_PER_SOL;


    try {
        const signature = await connection.requestAirdrop(publicKey, AIRDROP_AMOUNT);
        console.log("Airdrop transaction signature:", signature);
    } catch (error) {
        console.error("Error requesting airdrop:", error);
    }
    
    console.log("Airdrop successful!");
}

main();
