import { invoke } from "@tauri-apps/api";
import { writable } from "svelte/store";
import { raise_error } from "./walletError";

export const root_state = writable<object>(null);


export const getRootSate = async () => {
  console.log("getRootSate");
  return invoke("get_root_account", {})
    .then((res: object) => {
      root_state.set(res);
    })
    .catch((error) => {
      raise_error(error, true, "getRootSate"); // we have a purpose-built error component for this
    })
}