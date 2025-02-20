// import { Keypair } from "@solana/web3.js";
const { Keypair } = require("@solana/web3.js");

const privateKeyArray = [25, 250, 154, 253, 170, 251, 132, 37, 46, 128,   
    46, 49, 89, 8, 156, 186, 229, 154, 111,   
    26, 196, 50, 198, 218, 89, 140, 14, 199,   
    215, 55, 222, 164, 8, 234, 188, 216,   
    71, 250, 244, 237, 252, 154, 218, 63,   
    68, 81, 162, 103, 53, 14, 61, 180,   
    243, 70, 69, 157, 107, 136, 133, 38,   
    209, 239, 66, 247];  


// Convert the array to Uint8Array  
const privateKey = Uint8Array.from(privateKeyArray);  

// Create Keypair from secret key  
const keypair = Keypair.fromSecretKey(privateKey);  

// Convert private key to base58  
const privateKeyBase58 = Buffer.from(keypair.secretKey).toString('base64');  

// Output the public key and private key in base58 format  
console.log("Public Key:", keypair.publicKey.toBase58());  
console.log("Private Key (Base58):", privateKeyBase58);   
