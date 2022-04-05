<script lang="ts">
  import { onMount } from "svelte";
  import { Link, useLocation } from "svelte-navigator";
  import { signingAccount, isInit } from "../accounts";
  import AccountSwitcher from "./wallet/AccountSwitcher.svelte";
  import { routes } from "../routes";
  import { _ } from "../lang/i18n";
  import { init_preferences } from "../preferences";

  init_preferences();

  const secondaryRoutes = [
    routes.settings,
    routes.about,
    routes.developer,
    routes.keygen,
    routes.accountFromMnem,
  ];

  const location = useLocation();

  let myAccountIsOnChain = false; // assume initialized until not
  let init = false; // assume initialized until not
  onMount(async () => {
    isInit.subscribe((i) => (init = i));

    signingAccount.subscribe((myAccount) => {
      if (myAccount) {
        myAccountIsOnChain = myAccount.on_chain;
      }
    });
  });
</script>

<main class="uk-margin-top">
  <div
    uk-toggle="target: #offcanvas-nav"
  >
    <span uk-icon="icon: menu" />
  </div>



  {#if secondaryRoutes.includes($location.pathname)}
    <Link to={routes.home}
      ><span class="" uk-icon="icon: arrow-left; ratio: 2" /></Link
    >
  {/if}

  <div id="offcanvas-nav" uk-offcanvas="overlay: false; mode: push">

    <div class="uk-offcanvas-bar">
      <ul
        class="uk-nav uk-nav-default {init && myAccountIsOnChain
          ? ''
          : 'uk-invisible'}"
      >

        <li><Link to={routes.home}>{$_("nav.wallet")}</Link></li>
        <li><Link to={routes.transactions}>{$_("nav.transactions")}</Link></li>
        <li><Link to={routes.events}>{$_("nav.events")}</Link></li>
      </ul>
    </div>
  </div>

  <div class="uk-align-right">
      {#if init}
      <AccountSwitcher />
  {/if}
  </div>

</main>
