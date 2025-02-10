<script lang="ts">
    import type { PageProps } from "./$types";
    import { invoke } from "@tauri-apps/api/core";
    import { type ICollectionCardModel, type IPlugin, CollectionCardModel } from "$lib/utils/collection";
    import CollectionTitle from "$lib/components/CollectionTitle.svelte";
    import PluginSearch from "$lib/components/PluginSearch.svelte";
    import Icon from "$lib/icons/Icon.svelte";
    import { Icons } from "$lib/icons";
  import PluginCardList from "$lib/components/PluginCardList.svelte";

    let { data }: PageProps = $props();
    let defaultVal = new CollectionCardModel("", "", "", "");

    let searchVal = $state("");

    async function getCollection(): Promise<ICollectionCardModel> {
        return await invoke<ICollectionCardModel>('get_collection', { id: data.id });
    }

    async function getPlugins(): Promise<Array<IPlugin>> {
        return await invoke<Array<IPlugin>>('get_plugins', { collectionId: data.id });
    }
</script>

<div class="w-full h-full p-2 flex flex-col gap-2">
    {#await getCollection()}
        <CollectionTitle collection={defaultVal} isSkeleton={true}/>
    {:then collection}
        <CollectionTitle collection={collection} isSkeleton={false}/>
    {/await}
    <div class="w-full p-2 bg-neutral-900 rounded flex flex-row-reverse gap-2">
        <button class="bg-zinc-800 px-2 py-1 rounded hover:cursor-pointer hover:bg-zinc-700 flex flex-row gap-2 select-none">
            Install <Icon icon={Icons.Plus} colour="#FFFFFF"/>
        </button>
        <PluginSearch bind:searchValue={searchVal}/>
    </div>
    {#await getPlugins()}
        <p>waiting for plugins...</p>
    {:then plugins}
        <PluginCardList plugins={plugins}/>
        <!-- <div class="flex flex-col w-full flex-1">
            {#each plugins as plugin}
                <p>{plugin.id}, {plugin.name}, {plugin.source}, {plugin.api_url}</p>
            {/each}
        </div> -->
    {/await}
</div>
