import type { IPlugin } from "$lib/models";

export function iconSrc(plugin: IPlugin): string {
    return `https://gcdn.thunderstore.io/live/repository/icons/${plugin.ident}.png`;
}
