import { get } from "svelte/store";
import { loadAccounts } from "./accountActions";
import { isInit } from "./accounts";
import { getLedgerInfo } from "./networks";

export const walletTick = async () => {
  console.log("walletTick");
  // fetch a waypoint to see if we can connect to any fullnode.
  // If successful this will set the `network.connected` bool to true. And wallet will display a view.

  await loadAccounts()

  if (get(isInit)) {
    await getLedgerInfo() 
  }

}
