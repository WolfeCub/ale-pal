<script lang="ts">
    import { queryClient } from "./api/client";
    import type { UpdateBeverageRequest } from "./api/rspc";
    import "./app.css";

    import List from "./List.svelte";
    import Modal from "./Modal.svelte";
    import { QueryClientProvider } from "@tanstack/svelte-query";

    let shown = $state(false);
    let existing : UpdateBeverageRequest | null = $state(null);

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

<QueryClientProvider client={queryClient}>
    <main>
        <div class="container mx-auto flex flex-col items-center">
            <List openModal={edit} />

            <!-- TODO: Maybe not fixed but inside the container -->
            <div class="fixed bottom-5 right-5">
                <button
                    class="btn btn-info btn-circle font-bold text-xl"
                    onclick={openModal}>+</button
                >
            </div>
        </div>

        {#if shown}
            <Modal close={closeModal} existing={existing} />
        {/if}
    </main>
</QueryClientProvider>
