<script lang="ts">
    import { fade } from "svelte/transition";

    import type { ICollectionCardModel } from "$lib/utils/collection";
    import { getSubName } from "$lib/utils";

    let { collection, isSkeleton }: { collection: ICollectionCardModel, isSkeleton: boolean } = $props();

    let isActivateDisabled = $state(true);
</script>

<div class="w-full bg-neutral-900 rounded p-2 flex flex-row gap-2 select-none" transition:fade>
    {#if isSkeleton}
    <div class="size-20 rounded bg-zinc-800 flex items-center justify-center animate-pulse"></div>
    <div class="flex flex-col w-1/2 gap-1">
        <div class="w-32 h-5 bg-zinc-800 rounded animate-pulse"></div>
        <div class="w-64 h-3 bg-zinc-800 rounded animate-pulse"></div>
        <div class="w-40 h-3 bg-zinc-800 rounded animate-pulse"></div>
    </div>
    <div class="flex flex-row justify-end items-start flex-1">
        <div class="px-3 py-2 bg-zinc-800 text-zinc-800 rounded animate-pulse">Activate</div>
    </div>
    {:else}
    <div class="size-20 rounded bg-zinc-800 flex items-center justify-center">
        <p class="text-xs">{getSubName(collection.name)}</p>
    </div>
    <div class="flex flex-col w-1/2">
        <p class="text-lg">{collection.name}</p>
        <p class="text-sm text-zinc-500" title="The collection's uuid">{collection.id}</p>
        <p class="text-sm text-zinc-500">{collection.game}, {collection.game_version}</p>
    </div>
    <div class="flex flex-row justify-end items-start flex-1">
        <button class="px-3 py-2 rounded bg-blue-800 transition-all duration-150 hover:cursor-pointer hover:bg-blue-600
            disabled:bg-blue-900 disabled:cursor-not-allowed focus:outline-none"
            disabled={isActivateDisabled}>
            Activate
        </button>
    </div>
    {/if}
</div>
