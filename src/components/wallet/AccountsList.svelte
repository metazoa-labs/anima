<script lang="ts">
  import { _ } from "svelte-i18n";
  import type { AccountEntry } from "../../accounts";
  import UIkit from "uikit";
  import Icons from "uikit/dist/js/uikit-icons";
  import AccountCard from "./AccountCard.svelte";

  UIkit.use(Icons);

  // export let my_account: AccountEntry;
  export let account_list: AccountEntry[];
  // export let isConnected: boolean;

</script>

<main class="uk-margin">
  {#if account_list == null}
    <span uk-spinner />
  {:else if account_list.length > 0}
    <div class="uk-child-width-expand@s" uk-grid>
    {#each account_list as a, i}
      <AccountCard account={a.account} balance={a.balance}/>
    {/each}
    </div>
  {/if}
</main>
