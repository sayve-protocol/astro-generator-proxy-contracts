import "dotenv/config";
import { LCDClient, Wallet } from "@terra-money/terra.js";
import {
  newClient,
  readArtifact,
  writeArtifact,
  deployContract,
} from "./helpers.js";
import { join } from "path";
import { chainConfigs } from "./types.d/chain_configs.js";

const ARTIFACTS_PATH = "../artifacts";

async function main() {
  const { terra, wallet } = newClient();
  console.log(`chainID: ${terra.config.chainID} wallet: ${wallet.key.accAddress}`);

  if (!chainConfigs.generalInfo.multisig) {
    throw new Error("Set the proper owner multisig for the contracts")
  }

  await uploadAndInitGenProxyToSayve(terra, wallet)
}

async function uploadAndInitGenProxyToSayve(terra: LCDClient, wallet: Wallet) {
  const network = readArtifact(terra.config.chainID);

  if (!network.generatorProxyToSayveAddress) {
    console.log('Deploy the Generator proxy to sayve...');

    chainConfigs.proxySAYVE.admin ||= chainConfigs.generalInfo.multisig

    network.generatorProxyToSayveAddress = await deployContract(
      terra,
      wallet,
      chainConfigs.proxySAYVE.admin,
      join(ARTIFACTS_PATH, "generator_proxy_to_sayve.wasm"),
      chainConfigs.proxySAYVE.initMsg,
      chainConfigs.proxySAYVE.label,
    );

    console.log(`Address Generator proxy to SAYVE contract: ${network.generatorProxyToSayveAddress}`)
    writeArtifact(network, terra.config.chainID)
  }
}

main().catch(console.log);
