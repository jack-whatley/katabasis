<script lang="ts">
    import { DropdownMenu } from "bits-ui";
    import type { Snippet } from "svelte";

    interface DropdownItem {
        text: string;
        onclick: () => void;
    }

    interface Props {
        open?: boolean;
        children: Snippet;
        items: DropdownItem[];
    }

    let {
        open = $bindable(false),
        children,
        items,
    }: Props = $props();
</script>

<DropdownMenu.Root bind:open>
    <DropdownMenu.Trigger>
        {@render children()}
    </DropdownMenu.Trigger>
    <DropdownMenu.Portal>
        <DropdownMenu.Content
            class="bg-neutral-900 min-w-40 border border-neutral-800 m-2 p-1 rounded text-white text-xs">
            <DropdownMenu.Group>
                {#each items as item}
                    <DropdownMenu.Item textValue={item.text} onSelect={item.onclick}
                        class="px-4 py-2 rounded hover:bg-neutral-800 hover:cursor-pointer">
                        {item.text}
                    </DropdownMenu.Item>
                {/each}
            </DropdownMenu.Group>
        </DropdownMenu.Content>
    </DropdownMenu.Portal>
</DropdownMenu.Root>
