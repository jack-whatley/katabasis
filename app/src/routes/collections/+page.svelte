<script lang="ts">
  import CollectionCard from "$lib/components/CollectionCard.svelte";
    import { CollectionCardModel } from "$lib/utils/collection";

    let collections: Array<CollectionCardModel> = [
        new CollectionCardModel(
            "aaa", "Test Modpack", "Lethal Company"
        ),
        new CollectionCardModel(
            "bbb", "Example Modpack", "Lethal Company"
        ),
        new CollectionCardModel(
            "cccc", "Example Modpack with a really Long Name", "Lethal Company"
        ),
        new CollectionCardModel(
            "ddd", "Test Modpack", "Lethal Company"
        ),
        new CollectionCardModel(
            "eee", "Example Modpack", "Lethal Company"
        ),
        new CollectionCardModel(
            "fff", "Example Modpack with a really Long Name", "Lethal Company"
        )
    ];
</script>

<div class="w-full h-full p-2 flex flex-col">
    <p class="text-xl mb-2">Collections</p>
    <div class="flex-1 flex flex-row gap-2 flex-wrap justify-start content-start">
        {#each collections as collection}
            <CollectionCard collection={collection} />
        {/each}
    </div>
</div>
