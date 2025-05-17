<script lang="ts">
    interface TextInputProps {
        value: string
    }

    let {  
        value = $bindable(""),
        ...restProps
    }: TextInputProps = $props();
</script>

<input 
    type="search" bind:value {...restProps}
    autocapitalize="off" spellcheck="false" autocomplete="off"
    class={["bg-zinc-700 p-2 rounded text-xs focus:outline-4 focus:outline-blue-800", restProps.class]} />
