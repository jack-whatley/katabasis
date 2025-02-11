<script lang="ts">
    import type { Snippet } from "svelte";
    import { page } from "$app/state";

    let { children, url }: { children: Snippet, url: string } = $props();
    let isDisabled = $state(false);

    $effect(() => {
        if (url == "/") {
            isDisabled = page.url.pathname == url;
        }
        else {
            isDisabled = page.url.pathname.includes(url);
        }
    });
</script>

<a href={url} 
    class="flex flex-row mx-2 mt-2 p-2 gap-2 transition ease-in-out duration-75 rounded select-none hover:bg-blue-800 aria-disabled:bg-blue-800" 
    aria-disabled={isDisabled}>
    {@render children()}
</a>
