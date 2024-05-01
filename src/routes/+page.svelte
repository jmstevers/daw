<script>
    import { Button } from "$lib/components/ui/button";
    import { invoke } from "@tauri-apps/api/core";
    import { emit } from "@tauri-apps/api/event";
    import { Square } from "lucide-svelte";

    let recording = $state(false);

    async function start_recording() {
        recording = true;
        await invoke("start_recording");
    }

    async function stop_recording() {
        recording = false;
        await emit("stop_recording");
    }
</script>

<Button onclick={async () => await invoke("create_window")}>Open Window</Button>
<Button onclick={async () => await invoke("play_sound")}>Play Sound</Button>
<Button
    onclick={async () => {
        if (recording) {
            await stop_recording();
        } else {
            await start_recording();
        }
    }}
>
    {#if recording}
        Stop Recording
    {:else}
        Start Recording
    {/if}
</Button>
