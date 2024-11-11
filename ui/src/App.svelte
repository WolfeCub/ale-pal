<script lang="ts">
    import { queryClient } from "./api/client";
    import "./app.css";

    import Router from "./Router.svelte";
    import Navbar from "./Navbar.svelte";
    import Toast from "./Toast.svelte";
    import Home from "./Home.svelte";
    import Settings from "./Settings.svelte";
    import { QueryClientProvider } from "@tanstack/svelte-query";
    import type { Route } from "./routing.svelte";
    import Modal from "./Modal.svelte";
    import { modalState } from "./modal.svelte";

    let routes: Route[] = [
        { path: "/", component: Home },
        { path: "/settings", component: Settings },
    ];
</script>

<QueryClientProvider client={queryClient}>
    <main>
        <div class="container mx-auto flex flex-col items-center">
            <Navbar />
            <Router {routes} />
        </div>

        <Toast />

        <!-- TODO: Move this inside of modal. Figure out how to have $state inside react to `existing` -->
        {#if modalState.show }
            <Modal />
        {/if}
    </main>
</QueryClientProvider>
