<script lang="ts">
    import { onMount } from "svelte";
    import { Player } from "$lib/players";

    type VoidFn = () => void;

    export let gameWinner: Player;
    export let endGameFn: VoidFn;

    onMount(() => {
        setTimeout(() => {
            endGameFn();
        }, 5000);
    });
</script>

<div class="winner">
    <p class="game-over">Game over:&nbsp;</p>
    {#if gameWinner === Player.Empty}
        <p class="result-tie">it's a tie!</p>
    {:else if gameWinner === Player.P1}
        <p class="result-p1">you won!</p>
    {:else}
        <p class="result-bot">bot wins!</p>
    {/if}
</div>

<style>
    .winner {
        display: flex;
        justify-content: center;
    }
    .game-over {
        color: black;
        display: inline-block;
        font-size: 1.5em;
    }
    .result-tie {
        color: gray;
        display: inline-block;
        font-size: 1.5em;
    }
    .result-p1 {
        color: green;
        display: inline-block;
        font-size: 1.5em;
    }
    .result-bot {
        color: red;
        display: inline-block;
        font-size: 1.5em;
    }
</style>
