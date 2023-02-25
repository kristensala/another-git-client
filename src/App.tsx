import { onMount } from "solid-js";
import { invoke } from "@tauri-apps/api";
import SidePanel from "./components/SidePanel";
import GitHistory from "./components/GitHistory";
import "./styles/app.scss";
import Header from "./components/Header";

export default function App() {

    return (
        <main>
            <section class="section--header">
                <Header />
            </section>
            <section class="section--body">
                <div class="side-panel-container">
                    <SidePanel />
                </div>
                <div class="git-history-container">
                    <GitHistory />
                </div>
            </section>
        </main>
    );
};

