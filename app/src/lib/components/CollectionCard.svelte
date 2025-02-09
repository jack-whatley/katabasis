<script lang="ts">
    import type { ICollectionCardModel } from "$lib/utils/collection";
    import { getSubName } from "$lib/utils";

    let { collection }: { collection: ICollectionCardModel } = $props();
</script>

<a class="w-64 h-24 bg-neutral-900 p-2 rounded flex flex-row gap-2 hover:bg-blue-800" 
    title={collection.name}
    href={`/collections/${collection.id}`}>
    <div class="size-20 rounded bg-zinc-800 flex items-center justify-center">
        <p class="text-xs">{getSubName(collection.name)}</p>
    </div>
    <div class="flex-1 flex flex-col">
        <p class="text-md truncate w-36 select-none">{collection.name}</p>
        <p class="text-sm truncate w-36 select-none text-zinc-500">{collection.game}, {collection.game_version}</p>
    </div>
</a>
