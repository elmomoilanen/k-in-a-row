<script lang="ts">
    interface Props {
        message: string;
    }

    let { message }: Props = $props();
</script>

<div>
    <p class="emoji">☹</p>
    <p class="message">Unexpected error occurred: {message}</p>
    <p class="message">Please reload the page and try again.</p>
</div>

<style>
    div {
        background-color: var(--default-black);
        display: flex;
        position: fixed;
        flex-direction: column;
        margin: 0;
        top: 0%;
        width: 100%;
        height: 100%;
        z-index: 1;
    }
    div p {
        margin-bottom: 1em;
    }
    .message {
        color: white;
        font-size: 1.5rem;
        display: flex;
        justify-content: center;
        padding: 0.25em;
    }
    .emoji {
        color: white;
        font-size: 5rem;
        display: flex;
        justify-content: center;
        margin-top: 0.5em;
    }

    @media (prefers-color-scheme: dark) {
        div {
            background-color: var(--default-white);
        }
        .emoji {
            color: black;
        }
        .message {
            color: black;
        }
    }
</style>
