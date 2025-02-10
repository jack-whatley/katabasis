<script lang="ts">
    import type { IPlugin } from "$lib/utils/collection";

    let { plugin }: { plugin: IPlugin } = $props();
</script>

<div class="w-full h-15 py-2 grid grid-cols-[min-content_4fr_3fr_2fr] gap-3 items-center px-2 odd:bg-neutral-900 group">
    <div class="size-4 group-odd:bg-zinc-800 group-even:bg-neutral-900 mx-2 rounded"></div>
    <div title={plugin.id} class="flex flex-col">
        <p class="leading-5">{plugin.name}</p>
        <p class="text-xs">by Author</p>
    </div>
    <div><a href={plugin.api_url}>{plugin.source}</a></div>
    <div>Actions</div>
</div>
