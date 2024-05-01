<script>
    import { Button } from "$lib/components/ui/button";
    import { invoke } from "@tauri-apps/api/core";
    import { emit } from "@tauri-apps/api/event";
    import { Square, Circle } from "lucide-svelte";

    let recording = $state(false);
</script>

<Button onclick={async () => await invoke("create_window")}>Open Window</Button>
<Button onclick={async () => await invoke("play_sound")}>Play Sound</Button>
<Button onclick={async () => await invoke("record_sound")}>
    <Square />
</Button>
