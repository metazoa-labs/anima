import { writable } from "svelte/store";
import type { WalletError } from "./walletError";


// one of the Errors mapped in walletError.ts
// display these errors
// the state get switeched to false whenever a new backlog submission happens.
// todo: each error needs have its own rules for clearing

export const displayInsufficientBalance = writable<WalletError>({});
export const displayWrongDifficulty = writable<WalletError>({});
export const displayTooManyProofs = writable<WalletError>({});
export const displayDiscontinuity = writable<WalletError>({});
export const displayInvalidProof = writable<WalletError>({});

export const clearDisplayErrors = () => {
  displayWrongDifficulty.set({});
  displayTooManyProofs.set({});
  displayDiscontinuity.set({});
  displayInvalidProof.set({});
  displayInsufficientBalance.set({});
}

