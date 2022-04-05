<script lang="ts">
  import { _ } from "svelte-i18n";
  import type { AccountEntry } from "../../accounts";
  export let pendingAccounts: AccountEntry[];
  export let isConnected: boolean;

</script>

<main>
  {#if pendingAccounts.length > 0 && isConnected }
    <div class="uk-margin-large">
      <div class="uk-card uk-card-primary uk-card-hover uk-card-body uk-light">
        <h3 class="uk-card-title">{$_("wallet.reminder_create.card_title")}</h3>
        <p>{$_("wallet.reminder_create.message_headline")}</p>
        {#each pendingAccounts as a}
          <p> 
            {a.nickname} - {$_("wallet.reminder_create.onboard_key")} : <span class="uk-text-uppercase"> {a.authkey} </span>  
          </p>
        {/each}
      </div>
    </div>
  {/if}
</main>
