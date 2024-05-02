<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import { commands, events, type ValueLabel } from "$lib/bindings";
    import Combobox from "$lib/components/combobox.svelte";

    let recording = $state(false);

    async function getInputDevices(): Promise<ValueLabel[] | null> {
        const devices = await commands.getInputDevices();

        if (devices.status === "ok") {
            return devices.data;
        } else {
            return null;
        }
    }

    async function getOutputDevices(): Promise<ValueLabel[] | null> {
        const devices = await commands.getOutputDevices();

        if (devices.status === "ok") {
            return devices.data;
        } else {
            return null;
        }
    }

    $effect(() => {
        events.stopRecording.once((cb) => {
            console.log(cb.event);
        });
    });
</script>

<main class="flex h-screen flex-col items-center justify-center gap-4">
    <Button onclick={async () => await commands.createWindow()}>Open Window</Button>
    <Button onclick={async () => await commands.beep()}>Play Sound</Button>
    <Button
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
            Stop Recording
        {:else}
            Start Recording
        {/if}
    </Button>

    {#await getInputDevices()}
        <p>Loading...</p>
    {:then devices}
        <Combobox
            data={devices}
            placeholder="Select an input device"
            inputPlaceholder="Search input devices..."
        />
    {:catch error}
        <p>{error.message}</p>
    {/await}

    {#await getOutputDevices()}
        <p>Loading...</p>
    {:then devices}
        <Combobox
            data={devices}
            placeholder="Select an output device"
            inputPlaceholder="Search output devices..."
        />
    {:catch error}
        <p>{error.message}</p>
    {/await}
</main>
