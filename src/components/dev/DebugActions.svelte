<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { responses } from "../../debug";
  import DemoTx from "../txs/DemoTx.svelte";
  import { raise_error } from "../../walletError";
  import { listen } from '@tauri-apps/api/event'
  import { onMount } from "svelte";
  import { getCurrent } from '@tauri-apps/api/window'

  const makeError = async () => {
    invoke("debug_error", {
      debugErr: false,
    })
    .then((res) => responses.set(res))
    .catch((e) => raise_error(e, false, "makeError"));
  };

  const makeApiRequest = async () => {
    invoke("get_aptos_rest", {})
    .then((res) => responses.set(res))
    .catch((e) => raise_error(e, false, "get_aptos_rest"));
  };


  const triggerEventFromRustToJs = async () => {
    invoke("debug_emit_event", {})
    .then((res) => responses.set(res))
    .catch((e) => raise_error(e, false, "triggerEventFromRustToJs"));
  };

  function emitEventFromHereToRust() {

    // emit an event that are only visible to the current window
    const current = getCurrent();
    current.emit('emit-from-window', 'Tauri is awesome!');
  };
  const debugStartListener = async () => {
    invoke("debug_start_listener", {})
      .then((res) => {
        responses.set(res);
      })
      .catch((e) => console.error(e));
  };


  

  const init = async () => {
    invoke("init_user", {
      authkey: authkey_string,
      account: account_string,
      // pathStr: home,
    })
      .then((res) => {
        responses.set(res);
      })
      .catch((e) => console.error(e));
  };


  const testAsync = async () => {
    invoke("delay_async", {})
      .then((res) => {
        responses.set(res);
      })
      .catch((e) => console.error(e));
  };


    const startForever = async () => {
    invoke("start_forever_task", {})
      .then((res) => {
        responses.set(res);
      })
      .catch((e) => console.error(e));
  };

  const killForever = async () => {
    const current = getCurrent();
    current.emit('kill_forever', 'Tauri is awesome!');
  };
</script>

<main>
  <div>
    <div class="uk-margin-medium-bottom">
      <h4 class=" uk-text-uppercase  "> Helpers </h4>
      <button class="uk-button uk-button-default" on:click={makeError}>Make Error</button>
      <button class="uk-button uk-button-default" on:click={makeApiRequest}>API Request</button>
      <button class="uk-button uk-button-default" on:click={triggerEventFromRustToJs}>Receive Event</button>
      <button class="uk-button uk-button-default" on:click={debugStartListener}>Start Listener</button>
      <button class="uk-button uk-button-default" on:click={emitEventFromHereToRust}>Send Event</button>
    </div>


    <div class="uk-margin-medium-bottom">
      <h4 class=" uk-text-uppercase  "> Tests </h4>
      <button class="uk-button uk-button-default" on:click={testAsync}>Async</button>
      <button class="uk-button uk-button-default" on:click={startForever}>Start Forever</button>
      <button class="uk-button uk-button-default" on:click={killForever}>Kill Forever</button>
    </div>

    <div class="uk-margin-medium-bottom">
      <button class="uk-button uk-button-default" on:click={init}>Init</button>
    </div>

    <DemoTx />
  </div>
</main>
