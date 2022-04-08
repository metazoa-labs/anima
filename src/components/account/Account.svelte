<script lang="ts">
  import { _ } from "svelte-i18n";
  import { onDestroy, onMount } from "svelte";
  import Events from "../events/Events.svelte";
  import { signingAccount } from "../../accounts";
import { printCoins } from "../../coinHelpers";

  let unsubs = null;
  let account = null;

  onMount(async () => {
    unsubs = signingAccount.subscribe(value => account = value);
  });

  onDestroy(async () => {
    unsubs && unsubs();
  });
   
</script>
  
<main class="uk-height-viewport">
  <div class="uk-flex uk-flex-center">
    <h2 class="uk-text-uppercase">Account View</h2>
  </div>

  {#if account}
    <div class="uk-margin-bottom">
      <p><span class="uk-text-uppercase">Address: </span> <span class="uk-text-bold">{account.account}</span></p>
      <p><span class="uk-text-uppercase">Balance: </span> <span class="uk-text-bold">{printCoins(account.balance)}</span></p>
    </div>
    
    <Events {account} />
  {/if}
</main>