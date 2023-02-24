import { onMount } from "solid-js";
import { invoke } from "@tauri-apps/api";

export default function App() {
    
    onMount(() => {
        invoke("ping").then((res) => console.log(res));
    });

    function trigger(): void {
        invoke("test_command_line");
    }

    return (
        <div class="app-container">
            App rendered
            <button onclick={trigger}>Git log</button>
        </div>
    );
};

