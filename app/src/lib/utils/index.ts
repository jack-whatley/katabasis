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

function getSubName(name: string): string {
    let split = name.split(" ");
    let result = "";

    split.forEach((splitStr) => result += Array.from(splitStr)[0]);

    return result.toUpperCase();
}

export { SidebarItem, sidebarItems, settingsItem, getSubName };
