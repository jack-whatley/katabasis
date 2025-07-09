<script lang="ts">
    import type { ICollection } from "$lib/models";
    import NormalButton from "$lib/components/NormalButton.svelte";
    import { Plus, ArrowSquareIn, DotsThreeVertical } from "phosphor-svelte";
    import DropdownMenu from "$lib/components/DropdownMenu.svelte";
    import { invoke } from "@tauri-apps/api/core";

    interface Props {
        target: ICollection;
    }

    let {
        target
    }: Props = $props();

    let dropdownItems: Array<DropdownMenu.DropdownItem> = [
        { text: "Create Shortcut", onclick: shortcutCollection }
    ];

    async function launchCollection() {
        await invoke("launch_collection", { name: target.name });
    }

    async function shortcutCollection() {
        await invoke("shortcut_collection", { name: target.name });
    }
</script>

<div class="bg-neutral-900 p-3 rounded-md flex flex-row select-none items-center gap-2">
    <p class="text-lg">{target.name}</p>
    <div class="flex flex-row ml-auto gap-2 items-center">
        <div class="flex flex-row">
            <p class="text-xs border-r border-r-white px-2">{target.target}</p>
            <p class="text-xs px-2">{target.modLoader}</p>
        </div>
        <NormalButton onclick={() => {}}>
            <div class="flex flex-row items-center gap-1">
                <Plus size={16} />
                <p class="text-xs">Add Plugin</p>
            </div>
        </NormalButton>
        <NormalButton onclick={launchCollection}>
            <div class="flex flex-row items-center gap-1">
                <ArrowSquareIn size={16} />
                <p class="text-xs">Launch</p>
            </div>
        </NormalButton>
        <DropdownMenu items={dropdownItems}>
            <NormalButton onclick={() => {}}>
                <DotsThreeVertical size={16} />
            </NormalButton>
        </DropdownMenu>
    </div>
</div>
