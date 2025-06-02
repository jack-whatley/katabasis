<script lang="ts">
    import { Switch } from "bits-ui";

    let { checked = $bindable(), checkedCallback }: { checked: boolean, checkedCallback: () => void } = $props();
</script>

<Switch.Root
    class="peer inline-flex h-[24px] min-h-[24px] w-[42px] shrink-0 cursor-pointer items-center
    rounded-full transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-foreground 
    focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50 
    border border-neutral-700 data-[state=unchecked]:shadow-mini-inset data-[state=checked]:bg-foreground data-[state=unchecked]:bg-[#bf616a] 
    data-[state=checked]:bg-[#a3be8c]"
    bind:checked={checked}
    onCheckedChange={() => checkedCallback()}
    >
    <Switch.Thumb
        class="pointer-events-none block size-[20px] shrink-0 rounded-full transition-transform data-[state=checked]:translate-x-5 
        data-[state=unchecked]:translate-x-0 data-[state=unchecked]:shadow-mini bg-neutral-900 shadow-popover"
    />
</Switch.Root>
