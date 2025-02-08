<script lang="ts">
    import type { Snippet } from "svelte";

    let { children, isClose, clickCallback }: { children: Snippet, isClose: boolean, clickCallback: () => void } = $props();
</script>

{#if isClose}
<button onclick={() => clickCallback()} class="h-full max-w-fit p-2 cursor-pointer transition duration-150 ease-in-out hover:bg-[#bf616a]">
    {@render children()}
</button>
{:else}
<button onclick={() => clickCallback()} class="h-full max-w-fit p-2 cursor-pointer transition duration-150 ease-in-out hover:bg-[#a3be8c]">
    {@render children()}
</button>
{/if}
