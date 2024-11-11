<script lang="ts">
    import "./app.css";

    import List from "./List.svelte";
    import { modalState } from "./modal.svelte";

</script>

<List />
<!-- TODO: Maybe not fixed but inside the container -->
<div class="fixed bottom-5 right-5">
    <button
        class="btn btn-info btn-circle font-bold text-xl"
        onclick={() => modalState.open()}>+</button
    >
</div>
