<script lang="ts">
    import { Icons } from "$lib/icons";
    import Icon from "$lib/icons/Icon.svelte";

    let { searchValue = $bindable<string>("") } = $props();
</script>

<div class="flex flex-row flex-1 bg-zinc-800 p-1 rounded gap-1 select-none">
    <Icon icon={Icons.DocSearch} colour="#FFFFFF"/>
    <input 
        type="text" 
        bind:value={searchValue} 
        class="focus:outline-none flex-1"
        autocomplete="off" autocorrect="off" autocapitalize="off" spellcheck="false" placeholder="Search plugins..."/>
</div>
