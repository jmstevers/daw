<script>
    import { Button } from "$lib/components/ui/button";
    import { invoke } from "@tauri-apps/api/core";
</script>

<Button onclick={async () => await invoke("create_window")}>Open Window</Button>
<Button onclick={async () => await invoke("play_sound")}>Play Sound</Button>