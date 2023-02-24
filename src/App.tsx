import { onMount } from "solid-js";
import { invoke } from "@tauri-apps/api";

export default function App() {
    
    onMount(() => {
        invoke("ping").then((res) => console.log(res));
    });

    return (
        <div class="app-container">
            App rendered
        </div>
    );
};

