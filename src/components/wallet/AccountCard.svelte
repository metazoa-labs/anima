<script lang="ts">
  import { Link } from "svelte-navigator";
  import { routes } from "../../routes";

  import { printCoins } from "../../coinHelpers";
  import { _ } from "../../lang/i18n";
  import { setAccount } from "../../accountActions";

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
        {printCoins(balance)}
      </div>
    </div>
    <div class="uk-card-footer">
      <div>
        <Link to={routes.account}> 
          <span on:click={() => setAccount(account)} uk-icon="icon: forward; ratio: 1" alt="Send Transaction">
        </Link>
      </div>
    </div>
  </div>
</main>
