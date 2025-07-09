<script lang="ts">
    import type { ClassValue } from "svelte/elements";
    import type { Snippet } from "svelte";
    import { Button } from "bits-ui";
    import { X } from "phosphor-svelte";

    interface Props {
        value: string;
        class?: ClassValue;
        inputIcon?: Snippet;
        placeholder?: string;
    }

    let {
        value = $bindable(""),
        inputIcon,
        placeholder,
        ...props
    }: Props = $props();
</script>

<div class="bg-neutral-900 rounded-md flex flex-row items-center focus-within:outline-2 focus-within:outline-emerald-700">
    {#if inputIcon !== undefined}
        <div class="px-2">
            {@render inputIcon()}
        </div>
    {/if}
    <input type="search" bind:value autocapitalize="off" autocomplete="off" aria-autocomplete="none" spellcheck="false"
           placeholder={placeholder ?? ""}
           class={[
                "py-2 rounded-md text-sm flex-1 focus:outline-none",
                props.class]}/>
    {#if value.length > 0}
        <Button.Root class="p-1 m-1 rounded-md hover:bg-neutral-600 hover:cursor-pointer" onclick={() => value = ""}>
            <X size={16} />
        </Button.Root>
    {/if}
</div>
