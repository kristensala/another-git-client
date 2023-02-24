import { createSignal, For, onMount } from "solid-js";

type Commit = {
    Description: string,
    Date: string,
    Author: string
}

export default function GitHistory() {
    const [commits, setCommits] = createSignal<Commit[]>([]);

    onMount(() => {
        //TODO: setCommits run a tauri command
    });

    return (
        <div class="git-history-container">
            <table class="history-table">
                <tbody>
                    <tr>
                        <th>Graph</th>
                        <th>Description</th>
                        <th>Author</th>
                        <th>Date</th>
                    </tr>
                    <For each={commits()}>{(commit, _) =>
                        <tr>
                            <td></td>
                            <td>{commit.Description}</td>
                            <td>{commit.Author}</td>
                            <td>{commit.Date}</td>
                        </tr>
                    }</For>
                </tbody>
            </table>
        </div>
    );
}
