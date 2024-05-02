<script lang="ts">
    import Check from "lucide-svelte/icons/check";
    import ChevronsUpDown from "lucide-svelte/icons/chevrons-up-down";
    import { cn } from "$lib/utils.js";
    import { tick } from "svelte";
    import * as Popover from "./ui/popover";
    import { Button } from "./ui/button";
    import * as Command from "./ui/command/index";
    import type { ValueLabel } from "$lib/bindings";

    let {
        data = [],
        defaultValue = null,
        placeholder = "Select an item",
        inputPlaceholder = "Search items...",
        onSelect = (value: string) => {},
    }: {
        data: ValueLabel[];
        defaultValue: ValueLabel | null;
        placeholder: string;
        inputPlaceholder: string;
        onSelect: (value: string) => void;
    } = $props();

    let open = $state(false);
    let value = $state(defaultValue?.value ?? null);

    let selectedValue = $derived(data?.find((f) => f.value === value)?.label ?? placeholder);

    // We want to refocus the trigger button when the user selects
    // an item from the list so users can continue navigating the
    // rest of the form with the keyboard.
    function closeAndFocusTrigger(triggerId: string) {
        open = false;
        tick().then(() => {
            document.getElementById(triggerId)?.focus();
        });
    }
</script>

<Popover.Root bind:open let:ids>
    <Popover.Trigger asChild let:builder>
        <Button
            builders={[builder]}
            variant="outline"
            role="combobox"
            aria-expanded={open}
            class="w-[200px] justify-between "
        >
            <span class="truncate">{selectedValue}</span>
            <ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
        </Button>
    </Popover.Trigger>
    <Popover.Content class="w-[200px] p-0">
        <Command.Root>
            <Command.Input placeholder={inputPlaceholder} />
            <Command.Empty>No framework found.</Command.Empty>
            <Command.Group>
                {#each data! as item}
                    <Command.Item
                        value={item.value}
                        onSelect={(currentValue) => {
                            value = currentValue;
                            closeAndFocusTrigger(ids.trigger);
                            onSelect?.(currentValue);
                        }}
                    >
                        <Check
                            class={cn("mr-2 h-4 w-4", value !== item.value && "text-transparent")}
                        />
                        {item.label}
                    </Command.Item>
                {/each}
            </Command.Group>
        </Command.Root>
    </Popover.Content>
</Popover.Root>
