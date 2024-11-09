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

        <h2 class="text-2xl font-bold mb-4">Add a New Beer</h2>

        <form onsubmit={submit} class="space-y-4">
            <div>
                <label for="name" class="label-text">Beer Name</label>
                <input
                    type="text"
                    id="name"
                    bind:value={name}
                    class="input input-bordered w-full"
                    placeholder="Enter beer name"
                    required
                />
            </div>

            <div>
                <label for="beerType" class="label-text">Beer Type</label>
                <select
                    class="select select-bordered w-full"
                    bind:value={kind}
                    required
                >
                    <option value="" disabled selected>Select a type</option>
                    {#if $kindsQuery.isSuccess}
                        {#each $kindsQuery.data ?? [] as kind}
                            <option value={kind.kind_id}>{kind.name}</option>
                        {/each}
                    {/if}
                </select>
            </div>

            <div>
                <label for="brewery" class="label-text">Brewery</label>
                <select
                    class="select select-bordered w-full"
                    bind:value={producer}
                    required
                >
                    <option value="" disabled selected>Select a brewery</option>
                    {#if $producersQuery.isSuccess}
                        {#each $producersQuery.data ?? [] as producer}
                            <option value={producer.producer_id}
                                >{producer.name}</option
                            >
                        {/each}
                    {/if}
                </select>
            </div>

            <div>
                <label for="description" class="label-text">Description</label>
                <textarea
                    id="description"
                    bind:value={description}
                    class="textarea textarea-bordered w-full"
                    placeholder="Write a description"
                    rows="3"
                    required
                ></textarea>
            </div>

            <div>
                <label for="rating" class="label-text">Rating (Out of 10)</label
                >
                <input
                    type="number"
                    id="rating"
                    bind:value={rating}
                    class="input input-bordered w-full"
                    min="0"
                    max="10"
                    step="0.5"
                    required
                />
            </div>

            <div class="modal-action">
                <button type="submit" class="btn btn-primary">Save</button>
            </div>
        </form>
    </div>
</div>
