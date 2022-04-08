import { loadAccounts } from "./accountActions";
import { refreshWaypoint } from "./networks";

export const walletTick = async () => {
  console.log("walletTick");
  // fetch a waypoint to see if we can connect to any fullnode.
  // If successful this will set the `network.connected` bool to true. And wallet will display a view.
  
  await refreshWaypoint()

  await loadAccounts()

}
