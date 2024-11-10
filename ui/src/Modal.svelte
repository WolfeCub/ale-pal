<script lang="ts">
    import {
        deleteBeverageMutation,
        getKindsQuery,
        getProducersQuery,
        upsertBeverageMutation,
    } from "./api/client";
    import type { UpdateBeverageRequest } from "./api/rspc";
    import { firstFileToByteArray } from "./utils";

    interface Props {
        close: () => void;
        existing: UpdateBeverageRequest | null;
    }

    let props: Props = $props();

    let kind = $state(props.existing?.beverage.kind_id ?? "");
    let producer = $state(props.existing?.beverage.producer_id ?? "");
    let name = $state(props.existing?.beverage.name ?? "");
    let rating = $state(props.existing?.beverage.rating ?? 10);
    let description = $state(props.existing?.beverage.description ?? "");

    let image: number[] | null = $state(props.existing?.beverage.image ?? null);

    const kindsQuery = getKindsQuery();
    const producersQuery = getProducersQuery();
    const beverageMutation = upsertBeverageMutation();
    const deleteMutation = deleteBeverageMutation();

    const onImageChange = async (list: FileList | null) => {
        const byteArray = await firstFileToByteArray(list);
        image = byteArray;
    };

    const submit = () => {
        $beverageMutation.mutate({
            beverage_id: props.existing?.beverage_id ?? null,
            beverage: {
                name: name,
                producer_id: Number(producer),
                kind_id: Number(kind),
                rating: rating,
                description: description,
                image: image,
            },
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

            <input
                accept="image/png, image/jpeg"
                onchange={(e) => {
                    onImageChange(e.currentTarget.files);
                }}
                type="file"
                value="Test"
            />

            <div class="modal-action flex justify-between w-full">
                <button
                    class="btn btn-error"
                    onclick={() => {
                        props.existing?.beverage_id &&
                            $deleteMutation.mutate(props.existing.beverage_id);
                    }}>Delete</button
                >
                <button type="submit" class="btn btn-primary">Save</button>
            </div>
        </form>
    </div>
</div>
