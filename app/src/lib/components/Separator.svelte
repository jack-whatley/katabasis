<script lang="ts">
    let { horizontal }: { horizontal: boolean } = $props();
</script>

{#if horizontal}
<div class="w-full h-[1px] bg-black">
</div>
{:else}
<div class="w-[1px] h-full bg-black">
</div>
{/if}
