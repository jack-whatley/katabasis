<script lang="ts">
    import type { Snippet } from "svelte";
    import { page } from "$app/state";

    interface Props {
        children: Snippet;
        label: string;
        url: string;
    }

    let {
        children,
        label,
        url,
    }: Props = $props();

    function isDisabled(): boolean {
        if (url === "/") {
            return page.url.pathname === "/";
        }

        return page.url.pathname.includes(url);
    }
</script>

<a href={url} aria-disabled={isDisabled()}
    class="mt-2 mx-2 px-3 py-2 rounded-md focus:outline-1 focus:outline-emerald-700 hover:bg-emerald-700 aria-disabled:bg-emerald-600">
    <div class="flex flex-row items-center select-none gap-2">
        {@render children()}
        <p class="text-sm">{label}</p>
    </div>
</a>
