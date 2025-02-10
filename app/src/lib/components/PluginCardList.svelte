<script lang="ts">
    import type { IPlugin } from "$lib/utils/collection";
    import PluginCard from "./PluginCard.svelte";

    let { plugins = $bindable() }: { plugins: Array<IPlugin> } = $props();
</script>

<div class="rounded flex-1 flex flex-col">
    <div class="px-2 h-8 gap-3 grid grid-cols-[min-content_4fr_3fr_2fr] items-center shadow-md">
        <div class="size-4 bg-neutral-900 mx-2 rounded"></div>
        <div>Name</div>
        <div>Source</div>
        <div>Actions</div>
    </div>
    <div class="flex flex-col flex-[1_1_0] min-h-0 overflow-y-scroll scrollbar">
        <div class="border-b border-l border-t border-neutral-900 rounded">
            {#each plugins as plugin}
                <PluginCard plugin={plugin}/>
            {/each}
        </div>
    </div>
</div>
