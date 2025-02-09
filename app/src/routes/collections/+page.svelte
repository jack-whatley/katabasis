<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { fade } from "svelte/transition";

    import CollectionCard from "$lib/components/CollectionCard.svelte";
    import LoadingInfinite from "$lib/components/LoadingInfinite.svelte";
    import { type ICollectionCardModel } from "$lib/utils/collection";

    async function fetchCollections(): Promise<Array<ICollectionCardModel>> {
        return await invoke<Array<ICollectionCardModel>>('get_collections');
    }
</script>

<div class="w-full h-full p-2 flex flex-col">
    <p class="text-xl mb-2">Collections</p>
    {#await fetchCollections()}
        <div class="flex-1 flex items-center justify-center">
            <LoadingInfinite />
        </div>
    {:then collections}
        <div class="flex-1 flex flex-row gap-2 flex-wrap justify-start content-start" transition:fade>
            {#each collections as collection}
                <CollectionCard collection={collection} />
            {:else}
                <p>No collections exist...</p>
            {/each}
        </div>
    {:catch error}
        <p>Failed to fetch all collections: {error}</p>
    {/await}
</div>
