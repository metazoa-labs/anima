<script lang="ts">
  import { _ } from "svelte-i18n";
  import { onDestroy, onMount } from "svelte";
  import Events from "../events/Events.svelte";
  import { signingAccount } from "../../accounts";
  import { printCoins } from "../../coinHelpers";
  import { Link } from "svelte-navigator";
  import { routes } from "../../routes";

  let unsubs = null;
  let account = null;

  onMount(async () => {
    unsubs = signingAccount.subscribe(value => account = value);
  });

  onDestroy(async () => {
    unsubs && unsubs();
  });

  // only for muckup
  function printAddress(add) {
    return add ? add.substring(0,32) : add;
  }
   
</script>
  
<main>
  <div class="uk-flex uk-flex-center">
    <h2 class="uk-text-uppercase">Account View</h2>
  </div>

  {#if account}
    <div class="uk-margin-bottom uk-grid-small" uk-grid>
      <div class="uk-width-2-3">
        <p><span class="uk-text-uppercase">Address: </span> <span class="uk-text-bold">{printAddress(account.account)}</span></p>
        <p><span class="uk-text-uppercase">Balance: </span> <span class="uk-text-bold">{printCoins(account.balance)}</span></p>
      </div>
      <div class="uk-width-1-3 uk-text-right">
        <Link to={routes.transfer}> 
          <button class="uk-button uk-button-default" on:click={() => {}}>
            <span  uk-icon="icon: forward; ratio: 1" alt="Send Transaction"></span>
            Transfer Coins
          </button>        
        </Link>
      </div>      
    </div>
    
    <Events {account} />
  {/if}
</main>