/**
 * End-to-end test for the allowlist transfer hook.
 *
 * Loads the REAL Token-2022 program (dumped from mainnet) and the locally-built
 * hook program into LiteSVM, then drives an actual Token-2022 `transferChecked`
 * through the hook. Asserts:
 *   1. a transfer to an ALLOWLISTED destination SUCCEEDS,
 *   2. a transfer to a NON-allowlisted destination FAILS (the hook rejects it),
 *   3. a transfer that OMITS the hook's extra accounts FAILS.
 *
 * No mocking of the token program — this is the genuine transfer path.
 *
 * Prereq: `npm run fixtures` (dumps Token-2022) and a built hook .so
 * (`cargo build-sbf` in ../). Run with `npm test`.
 */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";
import assert from "node:assert/strict";
import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import {
  ExtensionType,
  TOKEN_2022_PROGRAM_ID,
  getMintLen,
  createInitializeTransferHookInstruction,
  createInitializeMintInstruction,
  getAssociatedTokenAddressSync,
  createAssociatedTokenAccountInstruction,
  createMintToInstruction,
  createTransferCheckedInstruction,
  createTransferCheckedWithTransferHookInstruction,
  getExtraAccountMetaAddress,
  unpackAccount,
} from "@solana/spl-token";
import { LiteSVM, FailedTransactionMetadata } from "litesvm";

const __dirname = dirname(fileURLToPath(import.meta.url));
const HOOK_SO = resolve(__dirname, "../target/deploy/transfer_hook_allowlist.so");
const HOOK_KP = resolve(__dirname, "../target/deploy/transfer_hook_allowlist-keypair.json");
const TOKEN22_SO = resolve(__dirname, "./fixtures/spl_token_2022.so");

const DECIMALS = 2;
const WHITELIST_LEN = 353n;

// ---- tiny test harness ---------------------------------------------------
let passed = 0;
const checks: string[] = [];
function check(label: string, cond: boolean) {
  if (!cond) throw new Error(`ASSERT FAILED: ${label}`);
  passed++;
  checks.push(`  ✓ ${label}`);
}

function keypairFromFile(path: string): Keypair {
  return Keypair.fromSecretKey(Uint8Array.from(JSON.parse(readFileSync(path, "utf8"))));
}

function main() {
  const svm = new LiteSVM();
  const hookProgram = keypairFromFile(HOOK_KP).publicKey;
  svm.addProgramFromFile(TOKEN_2022_PROGRAM_ID, TOKEN22_SO);
  svm.addProgramFromFile(hookProgram, HOOK_SO);

  const payer = new Keypair();
  svm.airdrop(payer.publicKey, 100_000_000_000n);

  // A LiteSVM-backed Connection shim so the spl-token hook resolver can read
  // accounts (it only calls getAccountInfo).
  const conn = {
    getAccountInfo: async (pubkey: PublicKey) => {
      const acc = svm.getAccount(pubkey);
      if (!acc) return null;
      const owner = acc.owner instanceof PublicKey ? acc.owner : new PublicKey(acc.owner);
      return {
        data: Buffer.from(acc.data),
        owner,
        lamports: Number(acc.lamports),
        executable: acc.executable,
        rentEpoch: Number((acc as { rentEpoch?: bigint }).rentEpoch ?? 0n),
      };
    },
  } as unknown as Connection;

  const send = (
    ixs: TransactionInstruction[],
    signers: Keypair[],
    label: string,
  ): boolean => {
    const tx = new Transaction();
    tx.add(...ixs);
    tx.feePayer = payer.publicKey;
    tx.recentBlockhash = svm.latestBlockhash();
    tx.sign(...signers);
    const res = svm.sendTransaction(tx);
    svm.expireBlockhash();
    const ok = !(res instanceof FailedTransactionMetadata);
    if (!ok && process.env.DEBUG_TX) {
      console.error(`--- tx "${label}" failed ---\n${(res as FailedTransactionMetadata).toString()}`);
    }
    return ok;
  };

  const balance = (ata: PublicKey): bigint => {
    const acc = svm.getAccount(ata)!;
    const info = {
      data: Buffer.from(acc.data),
      owner: acc.owner instanceof PublicKey ? acc.owner : new PublicKey(acc.owner),
      lamports: Number(acc.lamports),
      executable: acc.executable,
      rentEpoch: 0,
    };
    return unpackAccount(ata, info as never, TOKEN_2022_PROGRAM_ID).amount;
  };

  // 1) Create the Token-2022 mint with the TransferHook extension.
  const mintKp = new Keypair();
  const mint = mintKp.publicKey;
  const mintLen = getMintLen([ExtensionType.TransferHook]);
  const mintRent = Number(svm.minimumBalanceForRentExemption(BigInt(mintLen)));
  const okMint = send(
    [
      SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: mint,
        space: mintLen,
        lamports: mintRent,
        programId: TOKEN_2022_PROGRAM_ID,
      }),
      createInitializeTransferHookInstruction(mint, payer.publicKey, hookProgram, TOKEN_2022_PROGRAM_ID),
      createInitializeMintInstruction(mint, DECIMALS, payer.publicKey, null, TOKEN_2022_PROGRAM_ID),
    ],
    [payer, mintKp],
    "create mint",
  );
  check("mint created with transfer-hook extension", okMint);

  // 2) Initialize the hook's ExtraAccountMetaList (custom instruction, tag = 2).
  const extraMetaPda = getExtraAccountMetaAddress(mint, hookProgram);
  const metaRent = Number(svm.minimumBalanceForRentExemption(256n)); // generous; account is small
  const okMeta = send(
    [
      SystemProgram.transfer({ fromPubkey: payer.publicKey, toPubkey: extraMetaPda, lamports: metaRent }),
      new TransactionInstruction({
        programId: hookProgram,
        keys: [
          { pubkey: extraMetaPda, isSigner: false, isWritable: true },
          { pubkey: mint, isSigner: false, isWritable: false },
          { pubkey: payer.publicKey, isSigner: true, isWritable: false },
          { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        ],
        data: Buffer.from([2]),
      }),
    ],
    [payer],
    "init extra-account-meta-list",
  );
  check("ExtraAccountMetaList initialized", okMeta);

  // 3) Create + fund the allowlist PDA, then initialize it (tag = 0).
  const [whiteList] = PublicKey.findProgramAddressSync([Buffer.from("white-list")], hookProgram);
  const wlRent = Number(svm.minimumBalanceForRentExemption(WHITELIST_LEN));
  const okWl = send(
    [
      SystemProgram.transfer({ fromPubkey: payer.publicKey, toPubkey: whiteList, lamports: wlRent }),
      new TransactionInstruction({
        programId: hookProgram,
        keys: [
          { pubkey: whiteList, isSigner: false, isWritable: true },
          { pubkey: payer.publicKey, isSigner: true, isWritable: false },
          { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        ],
        data: Buffer.from([0]),
      }),
    ],
    [payer],
    "init whitelist",
  );
  check("allowlist account initialized", okWl);

  // 4) Source + two destinations.
  const srcOwner = payer; // mint authority also holds the source tokens
  const goodOwner = new Keypair();
  const badOwner = new Keypair();
  const srcAta = getAssociatedTokenAddressSync(mint, srcOwner.publicKey, false, TOKEN_2022_PROGRAM_ID);
  const goodAta = getAssociatedTokenAddressSync(mint, goodOwner.publicKey, false, TOKEN_2022_PROGRAM_ID);
  const badAta = getAssociatedTokenAddressSync(mint, badOwner.publicKey, false, TOKEN_2022_PROGRAM_ID);
  send(
    [
      createAssociatedTokenAccountInstruction(payer.publicKey, srcAta, srcOwner.publicKey, mint, TOKEN_2022_PROGRAM_ID),
      createAssociatedTokenAccountInstruction(payer.publicKey, goodAta, goodOwner.publicKey, mint, TOKEN_2022_PROGRAM_ID),
      createAssociatedTokenAccountInstruction(payer.publicKey, badAta, badOwner.publicKey, mint, TOKEN_2022_PROGRAM_ID),
      createMintToInstruction(mint, srcAta, payer.publicKey, 1000n, [], TOKEN_2022_PROGRAM_ID),
    ],
    [payer],
    "create ATAs + mint",
  );
  check("source minted 1000", balance(srcAta) === 1000n);

  // 5) Allowlist ONLY the good destination owner (tag = 1).
  send(
    [
      new TransactionInstruction({
        programId: hookProgram,
        keys: [
          { pubkey: whiteList, isSigner: false, isWritable: true },
          { pubkey: payer.publicKey, isSigner: true, isWritable: false },
        ],
        data: Buffer.concat([Buffer.from([1]), goodOwner.publicKey.toBuffer()]),
      }),
    ],
    [payer],
    "add good owner to allowlist",
  );

  // --- ASSERTION A: transfer to allowlisted dest SUCCEEDS ---
  return (async () => {
    const okIx = await createTransferCheckedWithTransferHookInstruction(
      conn, srcAta, mint, goodAta, srcOwner.publicKey, 100n, DECIMALS, [], undefined, TOKEN_2022_PROGRAM_ID,
    );
    const okTransfer = send([okIx], [payer], "transfer -> allowlisted");
    check("transfer to ALLOWLISTED destination succeeds", okTransfer);
    check("allowlisted destination received 100", balance(goodAta) === 100n);
    check("source debited by 100", balance(srcAta) === 900n);

    // --- ASSERTION B: transfer to NON-allowlisted dest FAILS ---
    const badIx = await createTransferCheckedWithTransferHookInstruction(
      conn, srcAta, mint, badAta, srcOwner.publicKey, 100n, DECIMALS, [], undefined, TOKEN_2022_PROGRAM_ID,
    );
    const badTransfer = send([badIx], [payer], "transfer -> non-allowlisted");
    check("transfer to NON-allowlisted destination fails (hook rejects)", badTransfer === false);
    check("non-allowlisted destination still 0", balance(badAta) === 0n);

    // --- ASSERTION C: transfer omitting hook extra accounts FAILS ---
    const plainIx = createTransferCheckedInstruction(
      srcAta, mint, goodAta, srcOwner.publicKey, 100n, DECIMALS, [], TOKEN_2022_PROGRAM_ID,
    );
    const plainTransfer = send([plainIx], [payer], "transfer without extra accounts");
    check("transfer WITHOUT hook extra accounts fails", plainTransfer === false);

    console.log("\nE2E transfer-hook allowlist — results:");
    console.log(checks.join("\n"));
    console.log(`\n${passed} checks passed.`);
  })();
}

main().catch((e) => {
  console.error(checks.join("\n"));
  console.error("\nE2E FAILED:", e.message);
  process.exit(1);
});
