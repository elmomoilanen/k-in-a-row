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

<div>
    <span class="winner-note">
        <p class="game-over">Game over:</p>
        {#if gameWinner === Player.Empty}
            <p class="result-tie">it's a tie!</p>
        {:else if gameWinner === Player.P1}
            <p class="result-p1">you won!</p>
        {:else}
            <p class="result-bot">bot wins!</p>
        {/if}
    </span>
</div>

<style>
    .winner-note {
        display: inline-block;
    }

    .game-over {
        color: black;
    }

    .result-tie {
        color: gray;
    }

    .result-p1 {
        color: green;
    }

    .result-bot {
        color: red;
    }
</style>
