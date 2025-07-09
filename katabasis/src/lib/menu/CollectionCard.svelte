<script lang="ts">
    import type { ICollection } from "$lib/models";

    interface Props {
        collection: ICollection;
    }

    let {
        collection
    }: Props = $props();
</script>

<a href="/library/collection?name={collection.name}"
   class="w-[150px] h-[75px] rounded-md px-2 py-1 bg-neutral-900 flex flex-col select-none hover:border-b-2 hover:border-emerald-700
        focus:border-b-2 focus:border-emerald-700 focus:outline-none">
    <p class="text-lg">{collection.name}</p>
    <p class="text-xs">{collection.target} | {collection.modLoader}</p>
</a>
