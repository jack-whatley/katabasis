<script lang="ts">
    import { Button } from "bits-ui";
    import type { Snippet } from "svelte";

    interface Props {
        children: Snippet;
        disabled?: boolean;
        callback: () => void;
    }

    let {
        children,
        disabled = $bindable(false),
        callback,
    }: Props = $props();
</script>

<Button.Root
    onclick={callback} disabled={disabled}
    class="bg-neutral-800 p-1 rounded-md disabled:bg-transparent hover:cursor-pointer hover:bg-neutral-700 z-30
        focus:outline-1 focus:outline-emerald-700">
    {@render children()}
</Button.Root>
