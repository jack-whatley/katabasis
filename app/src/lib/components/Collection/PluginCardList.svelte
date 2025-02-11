<script lang="ts">
    import type { IPlugin } from "$lib/utils/collection";
    import PluginCard from "./PluginCard.svelte";
    import { SortButton } from "..";

    let { plugins = $bindable(), searchValue = $bindable() }: { plugins: Array<IPlugin>, searchValue: string } = $props();

    let nameAsc = $state(true);
    let sourceAsc = $state(true);

    let actualList = $derived.by(() => {
        let list = plugins.filter(plugin => plugin.name.toLocaleLowerCase().includes(searchValue.toLocaleLowerCase()));

        if (nameAsc) {
            list.sort((a, b) => a.name.localeCompare(b.name));
        }
        else {
            list.sort((a, b) => b.name.localeCompare(a.name));
        }

        if (!sourceAsc) {
            list.sort((a, b) => a.source.localeCompare(b.source));
        }
        
        return list;
    });
</script>

<div class="rounded flex-1 flex flex-col">
    <div class="px-2 h-8 gap-3 grid grid-cols-[min-content_4fr_3fr_2fr] items-center select-none">
        <div class="size-4 bg-neutral-900 mx-2 rounded"></div>
        <SortButton bind:toggleable={nameAsc}>
            Plugin
        </SortButton>
        <SortButton bind:toggleable={sourceAsc}>
            Source
        </SortButton>
        <div>Actions</div>
    </div>
    <div class="w-full py-1 shadow-md"></div>
    <div class="flex flex-col flex-[1_1_0] min-h-0 overflow-y-scroll scrollbar">
        <div class="border-b border-l border-t border-neutral-900 rounded">
            {#each actualList as plugin}
                <PluginCard plugin={plugin}/>
            {/each}
        </div>
    </div>
</div>
