<script lang="ts">
    import type { ICollection } from "$lib/models";
    import { invoke } from "@tauri-apps/api/core";
    import CollectionMenu from "$lib/menu/CollectionMenu.svelte";
    import SearchInput from "$lib/components/SearchInput.svelte";
    import { MagnifyingGlass } from "phosphor-svelte";
    import PluginList from "$lib/plugins/PluginList.svelte";

    const urlParams = new URLSearchParams(window.location.search);
    const name = urlParams.get("name");

    async function loadCollection(): Promise<ICollection> {
        return await invoke<ICollection>("list_collection", { name: name });
    }

    let searchValue = $state("");
</script>

<div class="w-full h-full flex flex-col select-none p-2 gap-2">
    {#await loadCollection() then collection}
        <CollectionMenu target={collection} />
        <SearchInput bind:value={searchValue} placeholder={`Search ${collection.plugins.length} plugins...`}>
            {#snippet inputIcon()}
                <MagnifyingGlass size={16} />
            {/snippet}
        </SearchInput>
        <PluginList plugins={collection.plugins} bind:searchValue />
    {/await}
</div>
