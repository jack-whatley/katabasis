<script lang="ts">
    import { Icons, getIconPath } from ".";

    let { colour = "#e8eaed", width = "24px", height = "24px", icon }: { colour: string, width: string, height: string, icon: Icons } = $props();
    let iconPath = getIconPath(icon);
</script>

<svg xmlns="http://www.w3.org/2000/svg" width={width} height={height} viewBox="0 -960 960 960" fill={colour} color={colour}>
    <path d={iconPath}/>
</svg>
