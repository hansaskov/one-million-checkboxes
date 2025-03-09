<script lang="ts">
  import { store } from "./lib/store.svelte";

   store.startConnection();

  function onToggle(id: number) {
    store.connection?.reducers.toggle(id)
  }

</script>

{#each store.checkboxes as {id, checked} }
  <input type="checkbox" checked={checked} onclick={() => onToggle(id)} />
{/each}


