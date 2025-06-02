<script lang="ts">
	import { Select as SelectPrimitive, type WithoutChild } from "bits-ui";
	import CaretUpDown from "phosphor-svelte/lib/CaretUpDown";
	import { cn } from "$lib/utils.js";

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithoutChild<SelectPrimitive.TriggerProps> = $props();
</script>

<SelectPrimitive.Trigger
	bind:ref
	class={cn(
		"bg-zinc-700 ring-offset-background data-[placeholder]:text-muted-foreground flex w-full items-center " +
		 "justify-between rounded p-2 text-xs focus:outline-4 focus:outline-blue-800 " +
		  "disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1 hover:cursor-pointer",
		className
	)}
	{...restProps}
>
	{@render children?.()}
	<CaretUpDown class="size-4 opacity-50" />
</SelectPrimitive.Trigger>
