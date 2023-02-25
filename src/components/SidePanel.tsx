import { invoke } from "@tauri-apps/api";
import { createEffect, createSignal, For } from "solid-js";
import "../styles/side-panel.scss";

export default function SidePanel() {
    const [localBranches, setLocalBranches] = createSignal<string[]>([]);

    createEffect(() => {
        //todo: get local BRANCHES
    });

    function changeDir() {
        invoke("change_dir", {newDir: "asd"});
    }

    function getDir() {
        invoke("get_current_working_dir").then((res) => console.log(res));
    }

    return (
        <div class="side-panel-container">
            This is the left side panel
            <section class="panel-section--workspaces">
                <span>WORKSPACES</span>
            </section>

            <section class="panel-section--branches">
                <span>BRANCHES</span>
                <For each={localBranches()}>
                    {(branch, _) =>
                        <span>{branch}</span> 
                    }
                </For>
            </section>

            <section class="panel-section--stashes">
                <span>STASHES</span>
            </section>
            <button onclick={changeDir}>change dir</button>
            <button onclick={getDir}>get dir</button>
        </div>
    );
}
