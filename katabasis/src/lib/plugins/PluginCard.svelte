<script lang="ts">
    import type { IPlugin } from "$lib/models";
    import { iconSrc } from "$lib/utils";
    import Switch from "$lib/components/Switch.svelte";

    interface Props {
        plugin: IPlugin;
    }

    let {
        plugin
    }: Props = $props();

    let reactivePlugin = $state({
        enabled: plugin.enabled,
        installTime: plugin.installTime,
        ident: plugin.ident,
        fullName: plugin.fullName,
    });

    function parseName(name: string): string {
        let names = name.split("-");

        return names[1].charAt(0).toLocaleUpperCase() + names[1].slice(1);
    }
</script>

<div class="hover:bg-neutral-700 p-2 rounded-md flex flex-row items-center select-none gap-2">
    <img src={iconSrc(plugin)} alt={plugin.fullName} class="size-12 rounded-md" />
    <div class="flex flex-col">
        <p class={["text-lg", reactivePlugin.enabled ? "" : "line-through"]}>{parseName(plugin.fullName)}</p>
        <p class={["text-sm text-gray-300", reactivePlugin.enabled ? "" : "line-through"]}>{plugin.ident}</p>
    </div>
    <Switch class="ml-auto" bind:checked={reactivePlugin.enabled} />
</div>
