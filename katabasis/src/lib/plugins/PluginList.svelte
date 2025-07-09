<script lang="ts">
    import type { IPlugin } from "$lib/models";
    import PluginCard from "$lib/plugins/PluginCard.svelte";

    interface Props {
        plugins: Array<IPlugin>;
        searchValue: string;
    }

    let {
        plugins,
        searchValue = $bindable(""),
    }: Props = $props();

    let derivedPlugins = $derived(
        plugins.filter(
            x => x.fullName.toLocaleLowerCase().includes(searchValue.toLocaleLowerCase())));
</script>

<div class="flex-[1_1_0] overflow-y-auto">
    {#each derivedPlugins as plugin}
        <PluginCard plugin={plugin} />
    {/each}
</div>
