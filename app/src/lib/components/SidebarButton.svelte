<script lang="ts">
    import type { Snippet } from "svelte";
    import { page } from "$app/state";

    let { children, url }: { children: Snippet, url: string } = $props();
    let isDisabled = $state(page.url.pathname.includes(url));

    function isActive(): boolean {
        return false;
    }

    $effect(() => {
        // isDisabled = isActive();

        console.log(`button is ${isDisabled}`)
    });
</script>

<a href={url} 
    class="flex flex-row mx-2 mt-2 p-2 gap-2 transition ease-in-out duration-75 rounded select-none hover:bg-blue-800 disabled:bg-blue-800" 
    aria-disabled={isDisabled}>
    {@render children()}
</a>
