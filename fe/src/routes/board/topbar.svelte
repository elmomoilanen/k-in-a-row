<script lang="ts">
    export let boardSize: string;
    export let cellsToWin: number;
    export let goHome: () => void;

    let gameInfo = `${boardSize} ${cellsToWin.toFixed()}-in-a-row`;
</script>

<div class="topbar">
    <div
        id="home-icon"
        class="home-icon"
        aria-label="Go to app home view"
        role="button"
        tabindex="0"
        on:click={goHome}
        on:keydown
    >
        <svg
            class="bi bi-house"
            fill="currentColor"
            height="32"
            viewBox="0 0 16 16"
            width="32"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M8.707 1.5a1 1 0 0 0-1.414 0L.646 8.146a.5.5 0 0 0 .708.708L2 8.207V13.5A1.5 1.5 0 0 0 3.5 15h9a1.5 1.5 0 0 0 1.5-1.5V8.207l.646.647a.5.5 0 0 0 .708-.708L13 5.793V2.5a.5.5 0 0 0-.5-.5h-1a.5.5 0 0 0-.5.5v1.293L8.707 1.5ZM13 7.207V13.5a.5.5 0 0 1-.5.5h-9a.5.5 0 0 1-.5-.5V7.207l5-5 5 5Z"
            />
        </svg>
    </div>
    <p>{gameInfo}</p>
</div>

<style>
    .topbar {
        display: flex;
        flex-direction: row;
        position: absolute;
        top: 0%;
        width: 100%;
    }
    .topbar p {
        position: relative;
        font-size: 1.3rem;
        margin-top: 10px;
        left: 1%;
    }
    .topbar .home-icon {
        position: relative;
        left: 1%;
        top: 0%;
        cursor: pointer;
        margin-right: 2.5em;
    }
</style>
