/**
 * check-extensions.ts — inspect a Token-2022 mint and report its extensions + risks.
 *
 * Deterministic helper for the token-2022-extensions skill: don't guess what a
 * mint does, read it. Flags the extensions that change integration/safety
 * behaviour (transfer hook, transfer fee, permanent delegate, default-frozen,
 * non-transferable, pausable, confidential).
 *
 * Usage:
 *   npm i      (once)
 *   npx tsx check-extensions.ts <MINT_ADDRESS> [--url <RPC_URL>]
 *
 * Example (PYUSD, a real Token-2022 mint):
 *   npx tsx check-extensions.ts 2b1kV6DkPAnxd5ixfnxCpjxmKwqjjaYmCZfHsFu24GXo
 */
import { Connection, PublicKey } from "@solana/web3.js";
import {
  TOKEN_2022_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  ExtensionType,
  getExtensionTypes,
  getMint,
  getTransferFeeConfig,
  getTransferHook,
  getPermanentDelegate,
  getDefaultAccountState,
  AccountState,
} from "@solana/spl-token";

/** Extensions that materially change how integrators/holders must treat the token. */
const RISK: Partial<Record<ExtensionType, string>> = {
  [ExtensionType.TransferHook]:
    "Every transfer CPIs a program. Many DEXs BYPASS hooks — verify venue support; review the hook program.",
  [ExtensionType.TransferFeeConfig]:
    "Recipient gets amount-minus-fee. Use transfer_checked_with_fee; integrators must use net accounting.",
  [ExtensionType.PermanentDelegate]:
    "A permanent delegate can transfer/burn ANY holder's tokens — clawback / rug vector. Confirm who holds it.",
  [ExtensionType.DefaultAccountState]:
    "New accounts may be created FROZEN — airdrops/escrows/vaults break unless thawed.",
  [ExtensionType.NonTransferable]:
    "Soulbound — tokens cannot be transferred (can still be burned/closed).",
  [ExtensionType.MintCloseAuthority]:
    "Mint can be closed (supply must be 0) and the address reused.",
};

// Pausable / ScaledUiAmount / InterestBearing exist on newer enums; flag by name if present.
const RISK_BY_NAME: Record<string, string> = {
  Pausable: "A pause authority can halt all mint/burn/transfer. Integrators must handle paused state.",
  ScaledUiAmountConfig: "Displayed amount uses a multiplier; raw `amount` is unchanged — don't read raw as real.",
  ScaledUiAmount: "Displayed amount uses a multiplier; raw `amount` is unchanged — don't read raw as real.",
  InterestBearingConfig: "Interest is display-only; on-chain `amount` does not change.",
  ConfidentialTransferMint: "Confidential transfers depend on the ZK proof program (may be disabled on mainnet).",
};

function arg(name: string): string | undefined {
  const i = process.argv.indexOf(name);
  return i >= 0 ? process.argv[i + 1] : undefined;
}

async function main() {
  const mintArg = process.argv[2];
  if (!mintArg || mintArg.startsWith("--")) {
    console.error("Usage: tsx check-extensions.ts <MINT_ADDRESS> [--url <RPC_URL>]");
    process.exit(2);
  }
  const url = arg("--url") ?? "https://api.mainnet-beta.solana.com";
  const mint = new PublicKey(mintArg);
  const conn = new Connection(url, "confirmed");

  const info = await conn.getAccountInfo(mint);
  if (!info) {
    console.error(`Mint ${mintArg} not found on ${url}`);
    process.exit(1);
  }
  if (info.owner.equals(TOKEN_PROGRAM_ID)) {
    console.log(`${mintArg}\n→ Base SPL Token (no Token-2022 extensions). Widest compatibility.`);
    return;
  }
  if (!info.owner.equals(TOKEN_2022_PROGRAM_ID)) {
    console.error(`Owner ${info.owner.toBase58()} is not a known token program.`);
    process.exit(1);
  }

  const mintState = await getMint(conn, mint, "confirmed", TOKEN_2022_PROGRAM_ID);
  const tlv = (mintState as unknown as { tlvData: Buffer }).tlvData;
  const types = tlv && tlv.length > 0 ? getExtensionTypes(tlv) : [];

  const typeName = (t: ExtensionType): string => ExtensionType[t] ?? `type#${t}`;

  console.log(`Token-2022 mint: ${mintArg}`);
  console.log(`Decimals: ${mintState.decimals}  Supply: ${mintState.supply}`);
  console.log(`Extensions (${types.length}): ${types.map(typeName).join(", ") || "none"}\n`);

  if (types.length === 0) {
    console.log("No extensions enabled. Behaves like a base token.");
    return;
  }

  let riskCount = 0;
  for (const t of types) {
    const name = typeName(t);
    const note = RISK[t] ?? RISK_BY_NAME[name];
    if (note) {
      riskCount++;
      console.log(`⚠️  ${name}\n     ${note}`);
    } else {
      console.log(`•  ${name}`);
    }
  }

  // Surface concrete config for the highest-impact extensions.
  const fee = getTransferFeeConfig(mintState);
  if (fee) {
    const newer = fee.newerTransferFee;
    console.log(
      `\n   transfer fee: ${newer.transferFeeBasisPoints} bps, max ${newer.maximumFee} (epoch ${newer.epoch})`,
    );
  }
  const hook = getTransferHook(mintState);
  if (hook && !hook.programId.equals(PublicKey.default)) {
    console.log(`   transfer hook program: ${hook.programId.toBase58()}`);
  }
  const delegate = getPermanentDelegate(mintState);
  if (delegate && delegate.delegate && !delegate.delegate.equals(PublicKey.default)) {
    console.log(`   permanent delegate: ${delegate.delegate.toBase58()}`);
  }
  const das = getDefaultAccountState(mintState);
  if (das) {
    console.log(`   default account state: ${AccountState[das.state]}`);
  }

  console.log(
    `\n${riskCount} extension(s) need integration/safety attention. ` +
      `See compatibility-matrix.md and security.md before relying on this token.`,
  );
}

main().catch((e) => {
  console.error("Error:", e.message ?? e);
  process.exit(1);
});
