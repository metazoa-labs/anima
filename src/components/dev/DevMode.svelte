<script lang="ts">
  import DebugCard from "./DebugCard.svelte";
  import DebugActions from "./DebugActions.svelte";
  import DebugSwitchTestMainnet from "./DebugSwitchTestMainnet.svelte";
  import DebugSwitcher from "./DebugSwitcher.svelte";
</script>

<main class="uk-height-viewport">
  <div class="uk-margin uk-container uk-dark" style="margin-bottom: 280px">
    <DebugCard />
    <DebugSwitcher />
    <p class="uk-margin-bottom"> Turn on debug actions on all screens</p>
    <DebugSwitchTestMainnet />
    <DebugActions />
  </div>
</main>
