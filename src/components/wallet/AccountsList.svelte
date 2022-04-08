<script lang="ts">
  import { _ } from "svelte-i18n";
  import type { AccountEntry } from "../../accounts";
  import UIkit from "uikit";
  import Icons from "uikit/dist/js/uikit-icons";
  import AccountCard from "./AccountCard.svelte";
  UIkit.use(Icons);

  export let list: AccountEntry[];
  
</script>

<main class="uk-margin">
  {#if list == null}
    <span uk-spinner />
  {:else if list.length > 0}
    <div class="uk-child-width-expand@s" uk-grid>
      {#each list as a}
        <AccountCard account={a.account} balance={a.balance}/>
      {/each}
    </div>
  {/if}
</main>
