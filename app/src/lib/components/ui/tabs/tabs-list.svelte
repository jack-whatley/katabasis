<script lang="ts">
	import { Tabs as TabsPrimitive } from "bits-ui";
	import { cn } from "$lib/utils.js";

	let {
		ref = $bindable(null),
		class: className,
		...restProps
	}: TabsPrimitive.ListProps = $props();
</script>

<TabsPrimitive.List
	bind:ref
	class={cn(
		"w-full bg-zinc-700 text-muted-foreground rounded p-1 grid grid-cols-2 gap-1",
		className
	)}
	{...restProps}
/>
