import { Icons } from "$lib/icons";

class SidebarItem {
    display: string;
    url: string;
    icon: Icons;

    constructor(display: string, url: string, icon: Icons) {
        this.display = display;
        this.url = url;
        this.icon = icon;
    }
}

let sidebarItems: Array<SidebarItem> = [
    new SidebarItem(
        "Home", "/", Icons.Home
    ),
    new SidebarItem(
        "Collections", "/collections", Icons.Library
    )
]

let settingsItem: SidebarItem = new SidebarItem(
    "Settings", "/settings", Icons.Settings
);

export { SidebarItem, sidebarItems, settingsItem };
