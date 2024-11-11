<script lang="ts">
    import { toastState } from "./toast.svelte";
    $effect(() => {
        if (toastState.show) {
            setTimeout(() => toastState.close(), toastState.time);
        }
    });
</script>

{#if toastState.show}
    <div class="toast toast-top toast-end">
        <div class="alert alert-error">
            <span>{toastState.text}</span>
        </div>
    </div>
{/if}
