<script lang="ts">
    import type { UpdateBeverageRequest } from "./api/rspc";
    import "./app.css";

    import List from "./List.svelte";
    import Modal from "./Modal.svelte";

    let shown = $state(false);
    let existing: UpdateBeverageRequest | null = $state(null);

    const openModal = () => {
        shown = true;
    };

    const closeModal = () => {
        shown = false;
        existing = null;
    };

    const edit = (e: UpdateBeverageRequest | null) => {
        openModal();
        existing = e;
    };
</script>

<List openModal={edit} />

<!-- TODO: Maybe not fixed but inside the container -->
<div class="fixed bottom-5 right-5">
    <button
        class="btn btn-info btn-circle font-bold text-xl"
        onclick={openModal}>+</button
    >
</div>

{#if shown}
    <Modal close={closeModal} {existing} />
{/if}
