<script lang="ts">
    import { Select } from "bits-ui";
    import { type SelectorProps } from ".";

    import CaretUpDown from "phosphor-svelte/lib/CaretUpDown";
    import Check from "phosphor-svelte/lib/Check";

    let {
        selectValues,
        icon
    }: SelectorProps = $props();

    let triggerHtml = $state();
    let currentValue = $state<string>("");

    const currentLabel = $derived(
        currentValue 
            ? selectValues.find((item) => item.value === currentValue)?.label
            : "Select a game..."
    );
</script>

<Select.Root type="single" onValueChange={(v) => (currentValue = v)}>
    <Select.Trigger
        class="text-sm p-2 select-none flex flex-row items-center gap-2 border border-zinc-700 rounded"
        bind:innerHTML={triggerHtml}>
        {@render icon()}
        {currentLabel}
        <CaretUpDown size="20px" class="ml-auto"/>
    </Select.Trigger>
    <Select.Portal>
        <Select.Content
            class="focus-override z-55 max-h-96 w-[300px] select-none rounded border border-zinc700 bg-neutral-900"
            sideOffset={10}>
            <Select.Viewport>
                {#each selectValues as value}
                    <Select.Item
                        class="flex h-10 w-full select-none items-center rounded"
                        value={value.value}
                        label={value.label}>
                        {#snippet children({ selected })}
                            {value.label}
                            {#if selected}
                                <Check />
                            {/if}
                        {/snippet}
                    </Select.Item>
                {/each}
            </Select.Viewport>
        </Select.Content>
    </Select.Portal>
</Select.Root>
