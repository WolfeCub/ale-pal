<script lang="ts">
    import { type Component } from "svelte";
    import { createPathname, type Route } from "./routing.svelte";

    interface Props {
        routes: Route[];
    }

    let props: Props = $props();

    let lookup = props.routes.reduce(
        (acc, it) => {
            acc[it.path] = it.component;
            return acc;
        },
        {} as Record<string, Component>,
    );

    let pathname = createPathname();
</script>

<svelte:component this={lookup[pathname.path]} />
