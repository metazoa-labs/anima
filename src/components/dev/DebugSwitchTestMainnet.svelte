<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import type { WalletError } from "../../walletError";
  import { raise_error } from "../../walletError";
  import { notify_success } from "../../walletNotify";
  import {
    setNetwork,
    getNetwork,
    network_profile,
    NetworkID,
  ledger_info,
LedgerInfo,
  } from "../../networks";

  import type { NetworkProfile} from "../../networks";

  let current_chain_id = "";
  let waypoint = "";
  let li: LedgerInfo;

  function updateWaypoint() {
    // check input data
    // submit
    invoke("force_waypoint", { wp: waypoint })
      .then((res: NetworkProfile) => {
        network_profile.set(res);
        notify_success("Waypoint Updated");
      })
      .catch((error) => {
        raise_error(error as WalletError, false, "updateWaypoint");
      });
  }

  onMount(async () => {
    getNetwork();
    network_profile.subscribe((n) => {
      waypoint = n.waypoint;
      current_chain_id = n.chain_id;
    });

    ledger_info.subscribe((r: LedgerInfo) => li = r);
  });
</script>

<div class="uk-margin-medium-bottom">

  <h4 class=" uk-text-uppercase  ">
    Ledger Info
  </h4>
  {#if li && li.chain_id > 0}
  <div>
    <span>Chain id: {li.chain_id}</span>
    <span>Epoch: {li.epoch}</span>
    <span>Ledger Version: {li.ledger_version}</span>
    <span>Ledger Timestamp: {li.ledger_timestamp}</span>

  </div>
  {:else}
  <span>Cannot Connect To Network, No Ledger Info Found.</span>
  {/if}


  <h4 class=" uk-text-uppercase  ">
    Network Connection
  </h4>

  <!-- TODO: Change this to an interator of type NetworkID -->
  <div class="uk-margin uk-grid-small uk-child-width-auto uk-grid">
    {#each Object.values(NetworkID) as e}
        <label
      ><input
        class="uk-radio"
        type="radio"
        name="networkCb"
        checked={current_chain_id == e}
        on:click={() => setNetwork(e)}
      /> {e}
    </label>
      
    {/each}

    <!-- <label
      ><input
        class="uk-radio"
        type="radio"
        name="networkCb"
        checked={current_chain_id == "Testnet"}
      /> Testnet
        class="uk-radio"
        type="radio"
        name="networkCb"
        checked={current_chain_id == "Devnet"}
        on:click={() => setNetwork(NetworkID.Devnet)}
      /> Testnet
    </label>
        <label
      ><input
        class="uk-radio"
        type="radio"
        name="networkCb"
        checked={current_chain_id == "Local"}
        on:click={() => setNetwork(NetworkID.Local)}
      /> Testnet
    </label> -->
  </div>

  <h5 class=" uk-text-uppercase  ">
    Override The Waypoint
  </h5>
  <p>You need to know what you are doing here.</p>
  <div class="uk-margin uk-inline-block uk-width-1-1">
    <span> Waypoint </span>
    <input
      class="uk-input"
      type="text"
      placeholder={waypoint}
      bind:value={waypoint}
    />

  </div>

      <span
      on:click={updateWaypoint}
      class="uk-button uk-button-primary uk-align-right"
      id="add-btn">Override Waypoint</span
    >
</div>
