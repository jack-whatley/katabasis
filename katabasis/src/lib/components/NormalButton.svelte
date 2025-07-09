<script lang="ts">
    import type { Snippet } from "svelte";
    import { Button } from "bits-ui";
    import type { ClassValue } from "svelte/elements";

    interface Props {
        onclick: () => void;
        children: Snippet;
        class?: ClassValue;
    }

    let {
        onclick,
        children,
        ...props
    }: Props = $props();
</script>

<Button.Root onclick={onclick}
    class={[
		"rounded-md bg-emerald-600 p-2 hover:bg-emerald-700 hover:cursor-pointer active:scale-[0.98] transition-all",
		props.class ]}>
    {@render children()}
</Button.Root>
