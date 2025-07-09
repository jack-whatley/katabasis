<script lang="ts">
    import { Button } from "bits-ui";
    import { X, Minus, ArrowLeft, ArrowRight, Plus, Bell } from "phosphor-svelte";
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import IconButton from "$lib/components/IconButton.svelte";
    import NormalButton from "$lib/components/NormalButton.svelte";

    function onMinimise() {
        getCurrentWindow().minimize();
    }

    function onClose() {
        getCurrentWindow().close();
    }
</script>

<div data-tauri-drag-region
     class="h-[45px] flex flex-row items-center select-none border-b border-b-black">
    <div class="flex flex-row items-center gap-1 px-2">
        <IconButton callback={() => window.history.back()}>
            <ArrowLeft size={16} />
        </IconButton>
        <IconButton callback={() => window.history.forward()}>
            <ArrowRight size={16} />
        </IconButton>
    </div>
    <NormalButton onclick={() => {}} class="py-1 ml-auto mr-2 z-30">
        <div class="flex flex-row items-center gap-1">
            <Plus size={16} />
            <p class="text-xs">New Collection</p>
        </div>
    </NormalButton>
    <Button.Root
            onclick={onMinimise}
            class="h-full p-2 hover:bg-neutral-600 hover:cursor-pointer z-30">
        <Minus size={16} />
    </Button.Root>
    <Button.Root
            onclick={onMinimise}
            class="h-full p-2 hover:bg-neutral-600 hover:cursor-pointer z-30">
        <Bell size={16} />
    </Button.Root>
    <Button.Root
            onclick={onClose}
            class="h-full p-2 group hover:bg-neutral-600 hover:cursor-pointer z-30">
        <X size={16} />
    </Button.Root>
</div>
