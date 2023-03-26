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
        height: 30%;
        justify-content: center;
    }
    div p {
        color: white;
        font-size: 3rem;
        font-weight: bold;
        display: flex;
        justify-content: center;
        overflow-y: clip;
    }
    @media screen and (max-height: 500px) {
        div {
            height: 100%;
        }
    }
    @media (prefers-color-scheme: dark) {
        div {
            background-color: #131313;
        }
    }
</style>
