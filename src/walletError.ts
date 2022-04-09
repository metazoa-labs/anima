import { get, writable } from 'svelte/store';
import { notify_error } from './walletNotify';
export interface WalletError {
  category: number;
  uid: number;
  msg: string;
};

function empty_err(): WalletError{
  return {
    category: 0,
    uid: 0,
    msg: ""
  }
}

export enum ErrMap {
  NoClientCx = 404,
  AccountDNE = 1004,
}
// let list_errors: WalletError;
export const walletErrorLog = writable<[WalletError]>([]);

export function raise_error(err: WalletError, quiet: boolean = false, caller: string) {
  let hasCustomErrorDisplay = false;
  // maybe we need to take an action on this error type
  if (err.category) { // check this is the expected type
    // errAction(event.paylod);
    hasCustomErrorDisplay = errAction(err);
    err.msg = `${caller}: ${err.msg}`;
    console.log(err);
  } else {
    console.log(`WARN: ${caller}: error type returned is not a WalletError. Payload: ${err}`);
  }

  let list = get(walletErrorLog);
  list.push(err);
  walletErrorLog.set(list);
  console.log(list);
  let display = `Error (${err.uid}): ${err.msg}`

  if (!quiet && !hasCustomErrorDisplay) {
    notify_error(display);
  }
}


export function clearErrors() {
  walletErrorLog.set([]); // TODO: Assign this to empty array without causing TS error.
}

// returns true if there is a UI for the error, so we know to display generic error notification.
export const errAction = (err: WalletError): boolean => {
  switch (err.uid) {
    case ErrMap.NoClientCx:
      // window.alert("no client connection");
      return false // todo

    case ErrMap.AccountDNE:
      // window.alert("account does not exist");
      return false //todo


    default:
      return false
  }
}