
import { writable } from "svelte/store";
import type { WalletError } from "./walletError";


// one of the Errors mapped in walletError.ts
// display these errors
// the state get switeched to false whenever a new backlog submission happens.
// todo: each error needs have its own rules for clearing


export const displayInsufficientBalance = writable<WalletError>(null);
export const displayWrongDifficulty = writable<WalletError>(null);
export const displayTooManyProofs = writable<WalletError>(null);
export const displayDiscontinuity = writable<WalletError>(null);
export const displayInvalidProof = writable<WalletError>(null);

export const clearDisplayErrors = () => {
  displayWrongDifficulty.set(null);
  displayTooManyProofs.set(null);
  displayDiscontinuity.set(null);
  displayInvalidProof.set(null);
  displayInsufficientBalance.set(null);
}

