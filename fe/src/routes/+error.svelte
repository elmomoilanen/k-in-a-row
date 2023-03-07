<script lang="ts">
    import { page } from "$app/stores";

    interface ErrorMap {
        [key: number]: string | undefined;
    }

    const errorToEmoji: ErrorMap = {
        404: "\u1F61E",
        500: "\u2639"
    };
</script>

<h1>{$page.status} {$page.error?.message}</h1>
<span class="emoji">
    {errorToEmoji[$page.status] ?? errorToEmoji[500]}
</span>

<style>
    .emoji {
        font-size: 5em;
    }
</style>
