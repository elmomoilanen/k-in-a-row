<script lang="ts">
    export let showStart: boolean;
    export let showTime: number;

    setTimeout(() => {
        showStart = false;
    }, showTime);
</script>

<div>
    <p>You start</p>
</div>

<style>
    div {
        background-color: #484848;
        display: flex;
        position: fixed;
        top: 0%;
        width: 100%;
        z-index: 1;
        height: 20vh;
        justify-content: center;
    }
    div p {
        color: white;
        font-size: 3em;
        font-weight: bold;
        display: flex;
        justify-content: center;
    }

    @media (prefers-color-scheme: dark) {
        div {
            background-color: #131313;
        }
    }
</style>
