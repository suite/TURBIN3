import { Keypair } from "@solana/web3.js";
import bs58 from 'bs58'
import promptSync from 'prompt-sync';
const prompt = promptSync();

let kp = Keypair.generate();

console.log(`You've generated a new Solana wallet: ${kp.publicKey.toBase58()}`);
console.log(`[${kp.secretKey}]`);

function base58ToWallet() {
    let privateKey = prompt('Enter private key: (base58) ');
    let wallet = bs58.decode(privateKey);
    console.log(`[${wallet}]`);
}

function walletToBase58() {
    let privateKey = prompt('Enter private key: (array) ');
    let privateKeyArray = Uint8Array.from(JSON.parse(privateKey));
    let wallet = bs58.encode(privateKeyArray);
    console.log(`${wallet}`);
}

walletToBase58();
base58ToWallet();
