<script lang="ts">
    import { Icon, Icons } from "$lib/icons";
    import type { IPlugin } from "$lib/utils/collection";
    import { invoke } from "@tauri-apps/api/core";
    import Toggle from "../Toggle.svelte";
    import { Button } from "bits-ui";

    let { plugin, collectionId }: { plugin: IPlugin, collectionId: string } = $props();

    let enabled = $state(true);

    async function removePlugin() {
        await invoke('remove_plugins', { collectionId: collectionId, pluginId: plugin.id });
    }
</script>

<div class="w-full h-15 py-2 grid grid-cols-[min-content_4fr_3fr_1fr] gap-3 items-center px-2 odd:bg-neutral-900 group">
    <div class="size-4 group-odd:bg-zinc-800 group-even:bg-neutral-900 mx-2 rounded"></div>
    <div title={plugin.id} class="flex flex-col">
        <p class="leading-5">{plugin.name}</p>
        <p class="text-xs">by Author</p>
    </div>
    <div class="select-none"><a href={plugin.api_url}>{plugin.source}</a></div>
    <div class="flex flex-row gap-2 items-center justify-start">
        <Toggle bind:checked={enabled}/>
        <Button.Root 
            onclick={async () => await removePlugin()}
            class="group-odd:bg-zinc-800 group-even:bg-neutral-900 p-2 rounded hover:cursor-pointer active:scale-98 active:transition-all shadow-sm">
            <Icon icon={Icons.Bin} colour="#FFFFFF" width="16px" height="16px"/>
        </Button.Root>
    </div>
</div>
