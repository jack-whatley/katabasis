<script lang="ts">
    import { Icon, Icons } from "$lib/icons";
    import type { Snippet } from "svelte";

    let { children, toggleable = $bindable() }: { children: Snippet, toggleable: boolean } = $props();
</script>

<button onclick={() => toggleable = !toggleable}
    class="text-start hover:cursor-pointer flex flex-row items-center"
    title="Toggles grouping plugins by source">
    {@render children()}
    <div style={toggleable ? "rotate: 180deg;" : "rotate: 0deg;"} class="transition-transform duration-150 ease-in-out">
        <Icon icon={Icons.Down} colour="#FFFFFF" width="20px" height="20px"/>
    </div>
</button>
