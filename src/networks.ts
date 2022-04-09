import { invoke } from "@tauri-apps/api/tauri";
import { writable } from "svelte/store";
import { raise_error } from "./walletError";
import { loadAccounts } from "./accountActions";


  // Note: the string initialized should match the enum in Rust, networks.rs, to easily de/serialize
export enum NetworkID {
  Mainnet = "Mainnet",
  Testnet = "Testnet",
  Devnet = "Devnet",
  Local = "Local",
  Custom = "Custom"
}

export const network_profile = writable<NetworkProfile>({
  chain_id: NetworkID.Local, // Todo, use the Network Enum
  urls: ["string"],
  waypoint: "string",
  profile: "string",
});


export interface LedgerInfo {
  chain_id: number,
  epoch: number,
  ledger_version: number,
  ledger_timestamp: number,
}


export const ledger_info = writable<LedgerInfo>({
  chain_id: 0,
  epoch: 0,
  ledger_version: 0,
  ledger_timestamp: 0,
});

export let getLedgerInfo = () => {
  invoke("query_ledger_info", { })
    .then((res: LedgerInfo) => {
      ledger_info.set(res)
      connected.set(true);
      // loadAccounts(); // TODO notify as an event dependency
    })
    .catch((error) => {
      connected.set(false);
      ledger_info.set(null)
      raise_error(error, false, "getLedgerInfo");
    });
}

// should match the Rust type Network Profile
export interface NetworkProfile {
  chain_id: NetworkID, // Todo, use the Network Enum
  urls: [string],
  waypoint: string,
  profile: string,
}

export function setNetwork(network: NetworkID) {
  invoke("toggle_network", { network: network })
      .then((res: NetworkProfile) => {
        network_profile.set(res);
        // check if we are connected
        getLedgerInfo();
        // update accounts from current network
        loadAccounts(); // TODO notify as an event dependency
      })
    .catch((error) => raise_error(error, false, "setNetwork"));
}

export function getNetwork() {
  invoke("get_networks", {})
    .then((res: NetworkProfile) => network_profile.set(res))
    .catch((error) => raise_error(error, false, "getNetwork"));
}

export const connected = writable<boolean>(true);

// TODO: no API to get waypoint
export const refreshWaypoint = async () =>{
  console.log("refreshWaypoint");
  return invoke("refresh_waypoint", {})
    .then((res: NetworkProfile) => {
      network_profile.set(res);
      connected.set(true);
    })
    .catch((error) => {
      connected.set(false);
      raise_error(error, true, "refreshWaypoint"); // we have a purpose-built error component for this
    })
}