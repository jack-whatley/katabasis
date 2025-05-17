<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { page } from "$app/state";
    import { type ICollectionCardModel, type IPlugin, CollectionCardModel } from "$lib/utils/collection";
    import { CollectionTitle, PluginSearch, PluginCardList, LoadingInfinite } from "$lib/components";
    import { Icon, Icons } from "$lib/icons";
    import Dialogue from "$lib/components/Dialogue.svelte";

    import * as Select from "$lib/components/ui/select/index";
    import TextInput from "$lib/components/TextInput.svelte";
    import { Button } from "$lib/components/ui/button";

    let collectionId = page.url.searchParams.get("collectionId");

    let defaultVal = new CollectionCardModel("", "", "", "");

    let searchVal = $state("");

    let pluginSources = [
        { value: "thunderstore", label: "Thunderstore" },
    ];

    let selectedSource = $state("");
    let inputPluginUrl = $state("");

    const dropdownLabel = $derived(
        pluginSources.find((f) => f.value === selectedSource)?.label ?? "Select a game..."
    );

    async function getCollection(): Promise<ICollectionCardModel> {
        return await invoke<ICollectionCardModel>('get_collection', { id: collectionId });
    }

    async function getPlugins(): Promise<Array<IPlugin>> {
        return await invoke<Array<IPlugin>>('get_plugins', { collectionId: collectionId });
    }

    async function importPlugin(collectionId: string): Promise<void> {
        await invoke('import_plugin', { collectionId: collectionId, pluginUrl: inputPluginUrl });
    }
</script>

<div class="w-full h-full p-2 flex flex-col gap-2">
    {#await getCollection()}
        <CollectionTitle collection={defaultVal} isSkeleton={true}/>
    {:then collection}
        <CollectionTitle collection={collection} isSkeleton={false}/>
        <div class="w-full p-2 bg-neutral-900 rounded flex flex-row-reverse gap-2">
            <Dialogue title="Import Plugin">
                {#snippet button()}
                    <div class="bg-zinc-800 px-2 py-1 rounded hover:cursor-pointer hover:bg-zinc-700 flex flex-row gap-2 select-none">
                        Install <Icon icon={Icons.Plus} colour="#FFFFFF"/>
                    </div>
                {/snippet}
                {#snippet body()}
                    <div class="p-1">
                        <p class="text-lg">New Plugin</p>
                        <p class="text-sm mt-2 mb-1">Name</p>
                        <TextInput bind:value={inputPluginUrl} class="w-full" />
                        <p class="text-sm mt-2 mb-1">Plugin Source</p>
                        <Select.Root type="single" name="pluginSource" bind:value={selectedSource}>
                            <Select.Trigger class="w-full">
                                {dropdownLabel}
                            </Select.Trigger>
                            <Select.Content>
                                {#each pluginSources as source}
                                    <Select.Item value={source.value}>{source.label}</Select.Item>
                                {/each}
                            </Select.Content>
                        </Select.Root>
                        <Button variant="default" class="ml-auto w-26 mt-2"
                                onclick={async () => await importPlugin(collection.id)}>
                            Create
                        </Button>
                    </div>
                {/snippet}
            </Dialogue>
            <PluginSearch bind:searchValue={searchVal}/>
        </div>
    {/await}
    {#await getPlugins()}
        <div class="flex-1 flex items-center justify-center">
            <LoadingInfinite />
        </div>
    {:then plugins}
        <PluginCardList plugins={plugins} bind:searchValue={searchVal} collectionId={collectionId}/>
    {/await}
</div>
