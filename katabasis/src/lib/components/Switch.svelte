<script lang="ts">
    import { Switch } from "bits-ui";
    import type { ClassValue } from "svelte/elements";

    interface Props {
        checked: boolean;
        class?: ClassValue;
    }

    let {
        checked = $bindable(true),
        ...props
    }: Props = $props();

    let background = $derived(checked ? "bg-emerald-700" : "bg-rose-600");
</script>

<Switch.Root bind:checked class={[props.class, background, "p-2 rounded-full w-[60px] hover:cursor-pointer"]}>
    <Switch.Thumb
        class="size-3 bg-white block transition-all duration-150 data-[state=checked]:translate-x-[32px] data-[state=unchecked]:translate-x-0 rounded-full" />
</Switch.Root>
