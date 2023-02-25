import { createSignal, For, onMount } from "solid-js";
import "../styles/git-history.scss";

type Commit = {
    Description: string,
    Date: string,
    Author: string
}

export default function GitHistory() {
    const [commits, setCommits] = createSignal<Commit[]>([]);

    return (
        <div class="git-history-container">
            <table class="history-table">
                <tbody>
                    <tr>
                        <th style="width:5%">Graph</th>
                        <th>Description</th>
                        <th style="width:20%">Author</th>
                        <th style="width:20%">Date</th>
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
