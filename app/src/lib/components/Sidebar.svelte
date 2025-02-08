<script lang="ts">
    import AppTitle from "./AppTitle.svelte";
    import SidebarButton from "./SidebarButton.svelte";

    import { sidebarItems, settingsItem } from "$lib/utils";
    import { Icon } from "$lib/icons";
</script>

<div class="text-white min-w-1/6 bg-neutral-900 flex flex-col">
    <AppTitle/>
    {#each sidebarItems as item}
        <SidebarButton url={item.url}>
            <Icon colour="#FFFFFF" icon={item.icon}/>
            <p>{item.display}</p>
        </SidebarButton>
    {/each}
    <div class="mt-auto mb-2 w-full">
        <SidebarButton url={settingsItem.url}>
            <Icon colour="#FFFFFF" icon={settingsItem.icon}/>
            <p>{settingsItem.display}</p>
        </SidebarButton>
    </div>
</div>
