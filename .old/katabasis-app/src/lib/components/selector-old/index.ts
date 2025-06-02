import type { Snippet } from "svelte";

class SelectorItem {
    value: string;
    label: string;

    constructor(value: string, label: string) {
        this.value = value;
        this.label = label;
    }
}

interface SelectorProps {
    selectValues: Array<SelectorItem>,
    icon: Snippet
}

export { SelectorItem, type SelectorProps }
