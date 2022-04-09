<script lang="ts">
  import { _ } from "svelte-i18n";
  import { onDestroy, onMount } from "svelte";
  import { signingAccount } from "../../accounts"; 
  import { printCoins, printUnscaledCoins } from "../../coinHelpers";
  import { navigate } from "svelte-navigator";
  import UIkit from 'uikit';

  let unsubs = null;
  let account = null;
  let step = 1;

  let receiver = null;
  let amount = 0;
  let amountFormatted = "";
  let amountInput = null;

  onMount(async () => {
    unsubs = signingAccount.subscribe(value => account = value);
  });

  onDestroy(async () => {
    unsubs && unsubs();
  });

  /*
  Back button
  Steps 1 2 3 4

  Step 1: Enter the receiver address <next>
  Step 2: Enter the amount to be transfered <next>
  Step 3: Confirm transfer information <confirm>
  Step 4: Transfer result <OK>
  */

  function getStepClass(current: number, stepNumber: number) {
    if (stepNumber == current) {
      return "current-step";
    }
    if (stepNumber < current) {
      return "done-step";
    }
    return "next-step"
  }

  function handleChange() {
    let cleanedInput = amountInput.value
      .replace(/\D*/gm, '') // remove non digits
      .replace(/^0+/gm, ''); // remove leading zeros 

    if (cleanedInput.length === 0 ) {
      amount = 0;
      amountFormatted = '';
    } else {
      amount = parseInt(cleanedInput);
      amountFormatted = printUnscaledCoins(amount, 0, 0);
    }
  };

  let isWaiting = false;
  let errorCheck = "";
  const re = /[a-fA-F0-9]{32}/i;
  function nextClick() {
    if (step == 1) {
      if (!receiver) {
        return errorCheck = "You must type a receiver address.";
      }
      if (receiver == account.account) {
        return errorCheck = "You must type a receiver address different from your account address.";
      }
      if (!re.test(receiver)) {
        return errorCheck = "You must type a valid receiver address.";
      }
      errorCheck = "";
      return step++;
    }

    if (step == 2) {
      if (!amount) {
        return errorCheck = "You must enter an amount.";
      }
      if (amount > account.balance) {
        return errorCheck = "You must enter an amount smaller than the current account balance.";
      }
      return step++;
    }

    if (step == 3) {
      // send transaction
      isWaiting = true
      setTimeout(() => {
        isWaiting = false;
        UIkit.modal('#transferSuccess').show();
      }, 2000);
        
      // disable btns
      // await text

      // success -> modal -> account view
      // error -> warning msg -> step 3
      // finally -> enable buttons
    }
  };

  function backClick() {
    if (step == 1) {
      navigate("/account");
    }
    errorCheck = "";
    step--;
  };

</script>

<style>
  .current-step {
    border-color: orange;
  }
  .done-step {
    border-color: orange;
    background-color: orange;
  }
  .next-step {
    border-color: white;
  }
  .transfer-step {    
    border-style: solid;
    border-radius: 100%;
    width: 10px;
    height: 10px;
    margin: auto;
  }
  .success-icon{
    color: #4eb02e;
    font-weight: 900;
    border-radius: 100%;
    background-color: #dbf0d4;
    padding: 20px;
  }
</style>

<main class="uk-align-center" style="max-width: 500px;">
  <h2 class="uk-text-center uk-text-uppercase">
    Transfer Coins
  </h2>

  <div class="uk-text-center uk-margin-bottom">
    <div class="uk-flex" style="width: 140px; margin: auto;">
      <div class="transfer-step {getStepClass(step, 1)}"></div>
      <div class="transfer-step {getStepClass(step, 2)}"></div>
      <div class="transfer-step {getStepClass(step, 3)}"></div>
    </div>
  </div>

  <div class="uk-section">
    {#if step == 1}
      <div class="uk-margin-bottom">
        <p>Enter the address to transfer coins and click NEXT to continue.</p>
        <label class="uk-form-label" for="receiver-text">Receiver Address</label>
        <div class="uk-form-controls">
          <input
            id="receiver-text"
            autofocus
            class="uk-input"
            type="text"
            placeholder="Type a valid address"
            bind:value={receiver}
          />
        </div>
      </div>
    {:else if step == 2}
      <p>Enter the amount of coins you wnat to transfer and click NEXT.</p>
      <p><span class="uk-text-uppercase">Current balance: </span> <span class="uk-text-bold">{printCoins(account.balance)}</span></p>
      <label class="uk-form-label" for="amount-text">Amount to transfer</label>
      <div class="uk-form-controls uk-width-1-1">
        <!-- add mask -->
        <input
          id="amount-text"
          class="uk-input"
          type="text"
          placeholder="0.00"
          bind:value={amountFormatted}
          bind:this={amountInput}
          on:input={handleChange}
        />
      </div> 
    {:else if step == 3}
      <p>Now please review your transferm information and click CONFIRM to submit tramsaction.</p>
      <p><span class="uk-text-uppercase">Sender: </span> <span class="uk-text-bold">{account.account}</span></p>
      <p><span class="uk-text-uppercase">Receiver: </span> <span class="uk-text-bold">{receiver}</span></p>
      <p><span class="uk-text-uppercase">Amount: </span> <span class="uk-text-bold">{printUnscaledCoins(amount)}</span></p>
    {:else}
      <p>Ops. You are lost... Go back to Account view.</p>
    {/if}

    <div>
      <p class="uk-text-warning">{errorCheck}</p>
      <button 
        disabled={isWaiting}
        on:click={nextClick} 
        class="uk-button uk-button-primary uk-align-right"
      >
        {isWaiting 
          ? "Await"
          : step == 3 
            ? "Confirm" 
            : "Next"
        }
      </button>        
      <button on:click={backClick} class="uk-button uk-button-default uk-align-right">
        {step == 1 ? "Cancel" : "Back"}
      </button>   
      {#if isWaiting}     
        <div class="uk-align-right" uk-spinner></div>
      {/if}
    </div> 
  </div>

  <div id="transferSuccess" uk-modal>
    <div class="uk-modal-dialog uk-modal-body uk-text-center">
      <div class="uk-section">
        <h2 class="uk-modal-title"><span class="success-icon" uk-icon="icon: check; ratio: 2"></span></h2> 
        <p class="uk-text-small">Transation confirmed!</p>
        <p>You have transfered <span class="uk-text-bold">{amount ? printUnscaledCoins(amount) : ""} coins</span> for account <br>{receiver ? receiver : ""}.</p>
        <p>You can check your balance now.</p>
      </div>

      <p class="uk-text-center">
        <button 
          class="uk-button uk-button-large uk-button-primary uk-margin-right uk-modal-close" 
          type="button"
          on:click={() => navigate("/account")}
        >OK</button>
      </p>
    </div>
  </div>
</main>
