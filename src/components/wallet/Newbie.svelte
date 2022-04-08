<script  lang="ts">
  import { Link } from "svelte-navigator";
  import { routes } from "../../routes";
  import { init_preferences } from "../../preferences";
  import { _ } from "../../lang/i18n";
  
  init_preferences();
</script>

<main style="position:absolute" class="uk-position-center uk-margin-large">
  <img src="/anima-logo-white.svg" alt="drawing" width="100%"/>
    <div uk-grid class="uk-margin uk-flex uk-flex-center">
    <Link to={routes.keygen}>  
       <span uk-icon="icon: play-circle; ratio: 2"></span>
    </Link>
  </div>
</main>
