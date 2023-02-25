import { invoke } from "@tauri-apps/api";
import { createEffect, createSignal, For } from "solid-js";
import "../styles/side-panel.scss";


type Branch = {
    name: string,
    isActive: boolean
}

export default function SidePanel() {
    const [localBranches, setLocalBranches] = createSignal<Branch[]>([]);

    createEffect(() => {
        //todo: setLocalBranches
    });

    function changeDir() {
        invoke("change_dir", {newDir: "asd"});
    }

    function getDir() {
        invoke("get_current_working_dir").then((res) => console.log(res));
    }

    function getBranches() {
        invoke("git_branch_command").then((res) => {
            let list = res as Branch[];
            console.log(list);
        });
    }

    function changeBranch(newBranch: string): void {
        //TODO: trigger rust git checkout command
    }

    function isActiveBranch(branch: string): boolean {
        // todo: if branch starts with *, it is active
        return false;
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
                        <span onclick={() => changeBranch(branch.name)}>{branch.name}</span> 
                    }
                </For>
            </section>

            <section class="panel-section--stashes">
                <span>STASHES</span>
            </section>

            <button onclick={changeDir}>change dir</button>
            <button onclick={getDir}>get dir</button>
            <button onclick={getBranches}>localBranches</button>
        </div>
    );
}
