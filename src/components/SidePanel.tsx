import { invoke } from "@tauri-apps/api";
import { createEffect, onMount } from "solid-js";
import "../styles/side-panel.scss";

export default function SidePanel() {

    function changeDir() {
        invoke("change_dir", {newDir: "asd"});
    }

    function getDir() {
        invoke("get_current_working_dir").then((res) => console.log(res));
    }

    return (
        <div class="panel-wrapper">
            This is the left side panel
            <button onclick={changeDir}>change dir</button>
            <button onclick={getDir}>get dir</button>
        </div>
    );
}
