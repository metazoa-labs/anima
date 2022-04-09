<script lang="ts">
  import { onMount } from "svelte";
  import { getRootSate, root_state } from "../../queries";
  import CardAlert from "../layout/CardAlert.svelte";

  let state: object;

  let get = () => {
    getRootSate();
  }
  onMount(() => {
    root_state.subscribe(r => state = r);
  })
</script>

<main>
  <div class="uk-margin">
    <button class="uk-button uk-button-default" on:click={get}>Get 0x0 State</button>
  </div>

  {#if state } 
      <CardAlert>
      <span slot="title">System 0x0 State </span>
      <div slot="body">
        {JSON.stringify(state)}
      </div>
      </CardAlert>
  {/if}
</main>