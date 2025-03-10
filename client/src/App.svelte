<script lang="ts">
  import { onMount } from "svelte";
  import { store } from "./lib/store.svelte";

  // Virtual scrolling configuration
  let itemSize = $state(30);
  let viewportHeight = $state(500);
  let viewportWidth = $state(800);
  let scrollTop = $state(0);
  let buffer = $state(3);
  let viewport: HTMLElement;

  const length = 1_000_000
  
  // Calculate layout
  const itemsPerRow = $derived(Math.max(1, Math.floor(viewportWidth / itemSize)));
  const totalRows = $derived(Math.ceil(length / itemsPerRow));
  
  // Calculate visible range
  const startRow = $derived(Math.max(0, Math.floor(scrollTop / itemSize) - buffer));
  const endRow = $derived(Math.min(totalRows, Math.ceil((scrollTop + viewportHeight) / itemSize) + buffer));
  
  // Calculate visible items
  const startIndex = $derived(startRow * itemsPerRow);
  const endIndex = $derived(Math.min(length, (endRow + 1) * itemsPerRow));
  
  // Total height
  const contentHeight = $derived(totalRows * itemSize);

  function onToggle(id: number) {
    store.connection?.reducers.toggle(id);
  }

  onMount(() => {
    store.startConnection();
    
    if (viewport) {
      viewportHeight = viewport.clientHeight;
      viewportWidth = viewport.clientWidth;
      
      const observer = new ResizeObserver(() => {
        viewportHeight = viewport.clientHeight;
        viewportWidth = viewport.clientWidth;
      });
      
      observer.observe(viewport);
      return () => observer.disconnect();
    }
  });
</script>

<div 
  bind:this={viewport}
  onscroll={(e) => scrollTop = e.currentTarget.scrollTop}
  style="height: 100%; overflow-y: auto;"
>
  <div style="height: {contentHeight}px; position: relative;">
    
      {#each Array.from(store.checkboxes.entries()).slice(startIndex, endIndex) as [i, isChecked]}
        <input type="checkbox" checked={isChecked} onclick={() => onToggle(i)} />
      {/each}
  </div>
</div>