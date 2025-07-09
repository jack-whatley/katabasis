<script lang="ts">
    import CollectionCard from "$lib/menu/CollectionCard.svelte";
    import type { ICollection } from "$lib/models";
    import { invoke } from "@tauri-apps/api/core";

    async function loadCollections(): Promise<Array<ICollection>> {
        return await invoke<Array<ICollection>>("list_collections");
    }
</script>

<div class="w-full h-full flex flex-col select-none p-2 gap-2">
    <p class="text-2xl">Library</p>
    <div class="flex flex-row flex-wrap flex-[1_1_0] gap-2 overflow-y-auto">
        {#await loadCollections()}
            <p>Loading Collections...</p>
        {:then collections}
            {#each collections as collection}
                <CollectionCard collection={collection} />
            {/each}
        {:catch error}
            <p>Loading collections failed</p>
        {/await}
    </div>
</div>
