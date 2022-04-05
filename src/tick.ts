import { loadAccounts } from "./accountActions";
import { refreshWaypoint } from "./networks";

export const walletTick = async () => {
  console.log("walletTick");
  
  await refreshWaypoint()

  await loadAccounts()
}
