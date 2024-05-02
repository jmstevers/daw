<script lang="ts">
    import { commands, events, type ValueLabel } from "$lib/bindings";
    import { unwrap } from "$lib/utils";
    import Combobox from "$lib/components/combobox.svelte";
    import { Button } from "$lib/components/ui/button";
    import { Input } from "$lib/components/ui/input";
    import * as Sheet from "$lib/components/ui/sheet";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";

    import Package2 from "lucide-svelte/icons/package-2";
    import Menu from "lucide-svelte/icons/menu";
    import Search from "lucide-svelte/icons/search";
    import CircleUser from "lucide-svelte/icons/circle-user";
    import Square from "lucide-svelte/icons/square";
    import Circle from "lucide-svelte/icons/circle";
    import Play from "lucide-svelte/icons/play";
    import Pause from "lucide-svelte/icons/pause";

    let recording = $state(false);
    let playing = $state(false);
</script>

<div class="flex min-h-screen w-full flex-col">
    <header class="sticky top-0 flex h-16 items-center gap-4 border-b bg-background px-4 md:px-6">
        <nav
            class="hidden flex-col gap-4 text-lg font-medium md:flex md:flex-row md:items-center md:gap-5 md:text-sm"
        >
            <Button onclick={async () => await commands.createWindow()}>Open Window</Button>
            <Button
                size="icon"
                onclick={async () => {
                    playing = true;
                    await commands.beep();
                    playing = false;
                }}
            >
                {#if playing}
                    <Pause />
                {:else}
                    <Play />
                {/if}
            </Button>
            <Button
                size="icon"
                onclick={async () => {
                    if (recording) {
                        recording = false;
                        await events.stopRecording.emit();
                    } else {
                        recording = true;
                        await commands.record();
                    }
                }}
            >
                {#if recording}
                    <Circle />
                {:else}
                    <Square />
                {/if}
            </Button>
            {#await commands.getInputDevices() then devices}
                {#await commands.getCurrentInputDevice() then currentDevice}
                    <Combobox
                        data={unwrap(devices)}
                        defaultValue={unwrap(currentDevice)}
                        placeholder="Select an input device"
                        inputPlaceholder="Search input devices..."
                        onSelect={async (value: string) => {
                        await commands.setInputDevice(value);
                    }}
                    />
                {/await}
            {/await}

            {#await commands.getOutputDevices() then devices}
                {#await commands.getCurrentOutputDevice() then currentDevice}
                    <Combobox
                        data={unwrap(devices)}
                        defaultValue={unwrap(currentDevice)}
                        placeholder="Select an output device"
                        inputPlaceholder="Search output devices..."
                        onSelect={async (value: string) => {
                        await commands.setOutputDevice(value);
                    }}
                    />
                {/await}
            {/await}
        </nav>
        <Sheet.Root>
            <Sheet.Trigger asChild let:builder>
                <Button
                    variant="outline"
                    size="icon"
                    class="shrink-0 md:hidden"
                    builders={[builder]}
                >
                    <Menu class="h-5 w-5" />
                    <span class="sr-only">Toggle navigation menu</span>
                </Button>
            </Sheet.Trigger>
            <Sheet.Content side="left">
                <nav class="grid gap-6 text-lg font-medium">
                    <a href="##" class="flex items-center gap-2 text-lg font-semibold">
                        <Package2 class="h-6 w-6" />
                        <span class="sr-only">Acme Inc</span>
                    </a>
                    <a href="##" class="hover:text-foreground"> Dashboard </a>
                    <a href="##" class="text-muted-foreground hover:text-foreground"> Orders </a>
                    <a href="##" class="text-muted-foreground hover:text-foreground"> Products </a>
                    <a href="##" class="text-muted-foreground hover:text-foreground"> Customers </a>
                    <a href="##" class="text-muted-foreground hover:text-foreground"> Analytics </a>
                </nav>
            </Sheet.Content>
        </Sheet.Root>
        <div class="flex w-full items-center gap-4 md:ml-auto md:gap-2 lg:gap-4">
            <form class="ml-auto flex-1 sm:flex-initial">
                <div class="relative">
                    <Search class="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
                    <Input
                        type="search"
                        placeholder="Search products..."
                        class="pl-8 sm:w-[300px] md:w-[200px] lg:w-[300px]"
                    />
                </div>
            </form>
            <DropdownMenu.Root>
                <DropdownMenu.Trigger asChild let:builder>
                    <Button
                        builders={[builder]}
                        variant="secondary"
                        size="icon"
                        class="rounded-full"
                    >
                        <CircleUser class="h-5 w-5" />
                        <span class="sr-only">Toggle user menu</span>
                    </Button>
                </DropdownMenu.Trigger>
                <DropdownMenu.Content align="end">
                    <DropdownMenu.Label>My Account</DropdownMenu.Label>
                    <DropdownMenu.Separator />
                    <DropdownMenu.Item>Settings</DropdownMenu.Item>
                    <DropdownMenu.Item>Support</DropdownMenu.Item>
                    <DropdownMenu.Separator />
                    <DropdownMenu.Item>Logout</DropdownMenu.Item>
                </DropdownMenu.Content>
            </DropdownMenu.Root>
        </div>
    </header>
</div>
