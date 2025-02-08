<script lang="ts">
    import { Icons, getIconPath } from ".";

    let { colour = "#e8eaed", icon }: { colour: string, icon: Icons } = $props();
    let iconPath = getIconPath(icon);
</script>

<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px" fill={colour} color={colour}>
    <path d={iconPath}/>
</svg>
