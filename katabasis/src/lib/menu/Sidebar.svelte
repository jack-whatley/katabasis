<script lang="ts">
    import SidebarButton from "$lib/components/SidebarButton.svelte";
    import { House, Package, GearSix } from "phosphor-svelte";
</script>

<div class="w-1/5 h-full bg-neutral-900 border-r border-r-black flex flex-col select-none">
    <div class="flex h-[45px] items-center">
        <p class="px-3">katabasis</p>
    </div>
    <SidebarButton label="Home" url="/">
        <House size={16} />
    </SidebarButton>
    <SidebarButton label="Library" url="/library">
        <Package size={16} />
    </SidebarButton>
    <SidebarButton label="Settings" url="/settings">
        <GearSix size={16} />
    </SidebarButton>
    <p class="text-xs m-2 mt-auto">v0.0.1</p>
</div>
