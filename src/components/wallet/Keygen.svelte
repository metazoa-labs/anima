<script lang="ts">
  import { _ } from "svelte-i18n";
  import { onDestroy, onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { signingAccount, mnem } from "../../accounts";
  import type { AccountEntry } from "../../accounts";
  import { raise_error } from "../../walletError";
  import { responses } from "../../debug";
  import AccountFromMnemSubmit from "./AccountFromMnemSubmit.svelte";
  import { Link } from "svelte-navigator";
  import { routes } from "../../routes";

  interface NewKeygen {
    entry: AccountEntry;
    mnem: string;
  }

  let display_mnem: string;
  let address: string;
  let authkey: string;

  let unsubsMnem;
  let unsubsSigningAccount;

  onMount(async () => {
    unsubsMnem = mnem.subscribe((m) => (display_mnem = m));
    unsubsSigningAccount = signingAccount.subscribe((a) => {
      address = a.account;
      authkey = a.authkey;
    });
  });

  onDestroy(async () => {
    unsubsMnem && unsubsMnem();
    unsubsSigningAccount && unsubsSigningAccount();
  });

  let hide = true;
  const keygen = async () => {
    invoke("keygen", {})
      .then((res: NewKeygen) => {
        console.log(res);
        responses.set(JSON.stringify(res));
        signingAccount.set(res.entry);
        mnem.set(res.mnem);
        hide = false;
      })
      .catch((e) => raise_error(e, true, "keygen"));
  };
</script>

<main>
  <div class="uk-flex uk-flex-center">
    <h3 class="uk-text-light uk-text-muted uk-text-uppercase">
      {$_("wallet.keygen.title")}
    </h3>
  </div>

  {#if address && !hide}

  <div class="uk-child-width-expand@s uk-text-center" uk-grid>
    <div>
        <div class="uk-card uk-card-body">
      <h5 class="uk-text-muted uk-text-uppercase">
        {$_("wallet.keygen.account_address")}
      </h5>
      <h5 class="uk-text-break uk-text-emphasis uk-text-uppercase">{address}</h5>


        </div>
    </div>
    <div>
        <div class="uk-card  uk-card-body">
      <h5 class="uk-text-muted uk-text-uppercase uk-text-danger">
        {$_("wallet.keygen.securite_recovery_phrase")}
      </h5>
      <!-- <p class="uk-text-danger">
        {$_("wallet.keygen.securite_note")}
      </p> -->
      <div class="uk-margin">
        <textarea class="uk-textarea" rows="5" readonly>{display_mnem}</textarea
        >
      </div>

        </div>
    </div>
</div>

    <div>
      <AccountFromMnemSubmit danger_temp_mnem={""} />

      <button
        class="uk-button uk-button-default uk-align-right"
        on:click={keygen}
      >
        {$_("wallet.keygen.btn_generate_keys_2")}
      </button>
    </div>
  {:else}

    <div class="uk-position-center uk-flex uk-flex-center uk-flex-column uk-width-1-3">
      <div class="uk-text-center">
        <button
          class="uk-button uk-button-default"
          on:click={keygen}
        >
          {$_("wallet.keygen.btn_generate_keys")}
        </button>
      </div>
      <div class="uk-margin-top uk-text-center">
        <Link to={routes.accountFromMnem}>
          <!-- <button class="uk-button uk-button-default">{$_("wallet.btn_restore_account")}</button> -->
          <span class="uk-text-uppercase">
              <u> or Restore An Account </u></span
            >
        </Link>
      </div>
      <!-- <div class="uk-card uk-card-default uk-card-body uk-margin-top">Item 3</div> -->
    </div>

    <div class="" />
  {/if}
</main>
