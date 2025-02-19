<script lang="ts">
    import type { Snippet } from "svelte";
    import { Button, Dialog } from "bits-ui";
    import Draggable from "./App/Draggable.svelte";
    import X from "phosphor-svelte/lib/X";

    type DialogueProps = Dialog.RootProps & {
        title: string,
        button: Snippet,
        body: Snippet,
    };

    let {
        open = $bindable(false),
        title,
        button,
        body,
        ...restProps
    }: DialogueProps = $props();
</script>

<Dialog.Root bind:open>
    <Dialog.Trigger {...restProps}>
        {@render button()}
    </Dialog.Trigger>
    <Dialog.Portal>
        <Dialog.Overlay 
            class="fixed inset-0 z-50 bg-blue-800/5 backdrop-blur-xs">
            <Draggable />
        </Dialog.Overlay>
        <Dialog.Content
            class="fixed left-[50%] top-[50%] z-50 text-white w-[35%] translate-x-[-50%] translate-y-[-50%] rounded bg-neutral-900 shadow-md outline-none select-none"
            interactOutsideBehavior="defer-otherwise-close" trapFocus={false}>
            <Dialog.Title class="h-[45px] flex items-center px-2 border-b border-zinc-800">
                <p class="select-none text-lg">{title}</p>
                <Button.Root onclick={() => open = false} class="ml-auto p-1 rounded hover:bg-emerald-800 hover:cursor-pointer">
                    <X />
                </Button.Root>
            </Dialog.Title>
            <Dialog.Description class="select-none p-1">
                {@render body()}
            </Dialog.Description>
        </Dialog.Content>
    </Dialog.Portal>
</Dialog.Root>
