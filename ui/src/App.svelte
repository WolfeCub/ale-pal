<script lang="ts">
    import { queryClient } from "./api/client";
    import "./app.css";

    import List from "./List.svelte";
    import Modal from "./Modal.svelte";
    import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query";

    let shown = $state(false);
    const openModal = () => {
        shown = true;
    };
</script>

<QueryClientProvider client={queryClient}>
    <main>
        <div class="container mx-auto flex flex-col items-center">
            <List />

            <!-- TODO: Maybe not fixed but inside the container -->
            <div class="fixed bottom-5 right-5">
                <button
                    class="btn btn-info btn-circle font-bold text-xl"
                    onclick={openModal}>+</button
                >
            </div>
        </div>

        {#if shown}
            <Modal close={() => (shown = false)} />
        {/if}
    </main>
</QueryClientProvider>
