<script lang="ts">
    import {
        useKindsQuery,
        useProducersQuery,
        useBeverageMutation,
    } from "./api/client";

    interface Props {
        close: () => void;
    }

    let props: Props = $props();

    let kind = $state("");
    let producer = $state("");
    let name = $state("");
    let rating = $state(10);
    let description = $state("");

    const kindsQuery = useKindsQuery();
    const producersQuery = useProducersQuery();
    const beverageMutation = useBeverageMutation();

    const submit = () => {
        $beverageMutation.mutate({
            name: name,
            producer_id: Number(producer),
            kind_id: Number(kind),
            rating: rating,
            description: description,
        });

        props.close();
    };
</script>

<div id="modal" class="modal modal-open">
    <div class="modal-box">
        <form method="dialog">
            <button
                class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
                onclick={props.close}>✕</button
            >
        </form>

        <h3 class="text-lg font-bold">Add Beverage</h3>
        <input
            type="text"
            placeholder="Beverage Name"
            bind:value={name}
            class="input input-bordered w-full max-w-xs"
        />

        <select
            class="select select-bordered w-full max-w-xs"
            bind:value={kind}
        >
            <option value="" disabled selected>Type</option>
            {#if $kindsQuery.isSuccess}
                {#each $kindsQuery.data ?? [] as kind}
                    <option value={kind.kind_id}>{kind.name}</option>
                {/each}
            {/if}
        </select>

        <select
            class="select select-bordered w-full max-w-xs"
            bind:value={producer}
        >
            <option value="" disabled selected>Brewery</option>
            {#if $producersQuery.isSuccess}
                {#each $producersQuery.data ?? [] as producer}
                    <option value={producer.producer_id}>{producer.name}</option
                    >
                {/each}
            {/if}
        </select>

        <textarea
            class="textarea textarea-bordered"
            placeholder="Description"
            bind:value={description}
        ></textarea>

        <input
            type="number"
            placeholder="Rating"
            class="input input-bordered w-full max-w-xs"
            bind:value={rating}
        />

        <button class="btn" onclick={submit}>Button</button>
    </div>
</div>
