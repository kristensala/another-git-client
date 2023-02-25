import { onMount } from "solid-js";
import { invoke } from "@tauri-apps/api";
import SidePanel from "./components/SidePanel";
import GitHistory from "./components/GitHistory";
import "./styles/app.scss";

export default function App() {

    return (
        <div class="app-container">
            <div class="app-side-panel">
                <SidePanel />
            </div>
            <div class="app-history">
                <GitHistory />
            </div>
        </div>
    );
};

