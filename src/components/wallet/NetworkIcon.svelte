<script>
  import { onDestroy, onMount } from "svelte";
  import { NetworkID, network_profile } from "../../networks";

  let isTestNet = false;
  let unsubs;

  onMount(async () => {
    unsubs = network_profile.subscribe((network) => {
      isTestNet = network.chain_id == NetworkID.Testnet;
    });
  });

  onDestroy(async () => {
    unsubs && unsubs();
  });
</script>

{#if !isTestNet}
  <span uk-icon="icon: user"/>  
{:else}
    <!-- <img alt="test network icon" src="/images/crash-test.jpg"/> -->
  <span uk-icon="icon: warning"/>
  <span> TESTNET </span>
{/if}
