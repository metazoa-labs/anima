<script lang="ts">
  import { Link } from "svelte-navigator";
  import { routes } from "../../routes";
  import { onMount, onDestroy } from "svelte";
  import { accountEvents, signingAccount } from "../../accounts";
  import type { AccountEntry } from  "../../accounts";
  import { getAccountEvents } from "../../accountActions";
  import EventsTable from "./EventsTable.svelte";
  import EventsTableDummy from "./EventsTableDummy.svelte";
  import { _ } from "svelte-i18n";

  export let account: AccountEntry = null;
  
  let events = null;
  let unsubs;

  let loadingError = null;

  const errors = {
    "corrupted_db": $_("events.loading.corrupted_db"),
    "account_not_on_chain": $_("events.loading.account_off_chain"),
  }

  onMount(async () => {
    /*
    getAccountEvents(account, (error: string) => {
      loadingError = errors[error] || error;
    });
    unsubs = accountEvents.subscribe((all) => { 
      events = all[account.account] 
    });
    */
    events = [
      { transaction_version: 6, data: { e_type: "sentpayment", amount: { amount: 10000000 } , sender: "111111111111111" , receiver: account.account } },
      { transaction_version: 5, data: { e_type: "receivedpayment", amount: { amount: 7000000 } , sender: account.account, receiver: "111111111111111" } },
      { transaction_version: 4, data: { e_type: "sentpayment", amount: { amount: 21000000 } , sender: "111111111111111" , receiver: account.account } },
      { transaction_version: 3, data: { e_type: "sentpayment", amount: { amount: 15000000 } , sender: "111111111111111" , receiver: account.account } },
      { transaction_version: 2, data: { e_type: "sentpayment", amount: { amount: 6000000 } , sender: "111111111111111" , receiver: account.account } },
      { transaction_version: 1, data: { e_type: "sentpayment", amount: { amount: 3000000 } , sender: "111111111111111" , receiver: account.account } },
    ];
  });

  onDestroy(async () => {
    unsubs && unsubs();
  });

</script>
  
<main class="uk-height-viewport">
  <div style="position:relative">
    <div class="uk-flex uk-flex-left">
      <h4 class="uk-text-uppercase">Transfers</h4>
    </div>
    {#if loadingError}
      <p class="uk-text-center uk-text-warning">{$_("events.loading.error")}</p>
      <p class="uk-text-center uk-text-warning">{loadingError}</p>
      {#if !account.on_chain}
        <div style="position:absolute; top:0px; left:0px">
          <Link to={routes.home}><span class="" uk-icon="icon: arrow-left; ratio: 2;" /></Link>
        </div>
      {/if}
    {:else if events == null} 
      <span uk-spinner style="position:absolute; top:0px; left:0px"/>
      <EventsTableDummy />
    {:else}
      <EventsTable {events} />
    {/if}
  </div>
</main>