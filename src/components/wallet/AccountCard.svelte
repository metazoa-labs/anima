<script lang="ts">
import { Link } from "svelte-navigator";
  import { routes } from "../../routes";

  import { printCoins, unscaledCoins } from "../../coinHelpers";
  import { _ } from "../../lang/i18n";

  export let account;
  export let balance;
</script>

<main>
  <div class="uk-margin uk-card uk-card-default uk-light" style="background-color:rgba(255, 255, 255, 0.06)">
    <div class="uk-card-header">
      <h5 class="uk-card-title uk-text-muted uk-text-uppercase uk-text-truncate">
        {account}
      </h5>
    </div>
    <div class="uk-card-body">
      <div>
        {#if unscaledCoins(balance) < 1}
          <!-- TODO: make this icon align verical middle. -->
          <span class="uk-margin uk-text-warning" uk-icon="icon: info" />
          <div uk-dropdown>
            {$_("wallet.account_list.message")}
          </div>
        {/if}

        {printCoins(balance)}
      </div>
    </div>
    <div class="uk-card-footer">
      <div>

        <!-- TODO: Link <Link to={routes.transactions}> </Link> -->
          <span uk-icon="icon: forward; ratio: 1" alt="Send Transaction">
            <!-- <div uk-dropdown>Send A Transaction</div> -->
          </span>
       
          <!-- TODO: Link
        <Link to={routes.events}></Link> -->
          <span uk-icon="icon: list; ratio: 1" alt="View Events">
            <div uk-dropdown>View All Events</div>
          </span>
        
      </div>
    </div>
  </div>
</main>
