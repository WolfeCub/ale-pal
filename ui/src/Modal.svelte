<script lang="ts">
    import {
        deleteBeverageMutation,
        getKindsQuery,
        getProducersQuery,
        insertKindMutation,
        insertProducerMutation,
        upsertBeverageMutation,
    } from "./api/client";
    import { firstFileToByteArray } from "./utils";
    import { modalState } from "./modal.svelte";

    let kind = $state(modalState.existing?.beverage.kind_id ?? "");
    let producer = $state(modalState.existing?.beverage.producer_id ?? "");
    let name = $state(modalState.existing?.beverage.name ?? "");
    let description = $state(modalState.existing?.beverage.description ?? "");
    let rating = $state(modalState.existing?.beverage.rating ?? 5);

    let newKind = $state("");
    let showNewKind = $state(false);
    let newProducer = $state("");
    let showNewProducer = $state(false);

    let image: number[] | null = $state(
        modalState.existing?.beverage.image ?? null,
    );

    const kindsQuery = getKindsQuery();
    const producersQuery = getProducersQuery();
    const beverageMutation = upsertBeverageMutation();
    const deleteMutation = deleteBeverageMutation();
    const kindMutation = insertKindMutation();
    const producerMutation = insertProducerMutation();

    const onImageChange = async (list: FileList | null) => {
        const byteArray = await firstFileToByteArray(list);
        image = byteArray;
    };

    let fileUploadInputElement: HTMLInputElement | undefined = $state();

    $effect(() => {
        if (!fileUploadInputElement || !modalState.existing?.beverage.image)
            return;

        const dataTransfer = new DataTransfer();
        dataTransfer.items.add(new File([], "existing_image.png"));
        fileUploadInputElement.files = dataTransfer.files;
    });

    const submit = () => {
        $beverageMutation.mutate({
            beverage_id: modalState.existing?.beverage_id ?? null,
            beverage: {
                name: name,
                producer_id: Number(producer),
                kind_id: Number(kind),
                rating: rating,
                description: description,
                image: image,
            },
        });

        modalState.close();
    };

    const newKindEvent = (e: MouseEvent) => {
        e.preventDefault();

        if (showNewKind && newKind != "") {
            $kindMutation.mutate(
                { name: newKind },
                {
                    onSuccess: (kindId) => {
                        kind = kindId;
                    },
                },
            );
        }

        showNewKind = !showNewKind;
        newKind = "";
    };

    const newProducerEvent = (e: MouseEvent) => {
        e.preventDefault();

        if (showNewProducer && newProducer != "") {
            $producerMutation.mutate(
                { name: newProducer },
                {
                    onSuccess: (producerId) => {
                        producer = producerId;
                    },
                },
            );
        }

        showNewProducer = !showNewProducer;
        newProducer = "";
    };
</script>

<div id="modal" class="modal modal-open">
    <div class="modal-box">
        <form method="dialog">
            <button
                class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
                onclick={() => modalState.close()}>✕</button
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
                <label for="beerType" class="label-text"
                    >{showNewKind ? "Add New Type" : "Beer Type"}</label
                >
                <div class="flex">
                    {#if !showNewKind}
                        <select
                            class="select select-bordered w-full"
                            bind:value={kind}
                            required
                        >
                            <option value="" disabled selected
                                >Select a type</option
                            >
                            {#if $kindsQuery.isSuccess}
                                {#each $kindsQuery.data ?? [] as kind}
                                    <option value={kind.kind_id}
                                        >{kind.name}</option
                                    >
                                {/each}
                            {/if}
                        </select>
                    {:else}
                        <input
                            type="text"
                            bind:value={newKind}
                            class="input input-bordered w-full"
                            placeholder="Enter type name"
                            required
                        />
                    {/if}
                    <button class="btn" onclick={newKindEvent}>
                        {showNewKind
                            ? newKind != ""
                                ? "Add"
                                : "Select"
                            : "Edit"}
                    </button>
                </div>
            </div>

            <div>
                <label for="brewery" class="label-text">Brewery</label>
                <div class="flex">
                    {#if !showNewProducer}
                        <select
                            class="select select-bordered w-full"
                            bind:value={producer}
                            required
                        >
                            <option value="" disabled selected
                                >Select a brewery</option
                            >
                            {#if $producersQuery.isSuccess}
                                {#each $producersQuery.data ?? [] as producer}
                                    <option value={producer.producer_id}
                                        >{producer.name}</option
                                    >
                                {/each}
                            {/if}
                        </select>
                    {:else}
                        <input
                            type="text"
                            bind:value={newProducer}
                            class="input input-bordered w-full"
                            placeholder="Enter brewery name"
                            required
                        />
                    {/if}
                    <button class="btn" onclick={newProducerEvent}>
                        {showNewProducer
                            ? newProducer != ""
                                ? "Add"
                                : "Select"
                            : "Edit"}
                    </button>
                </div>
            </div>

            <div>
                <label for="description" class="label-text">Description</label>
                <textarea
                    id="description"
                    bind:value={description}
                    class="textarea textarea-bordered w-full"
                    placeholder="Write a description"
                    rows="3"
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
                bind:this={fileUploadInputElement}
                accept="image/png, image/jpeg"
                class="file-input file-input-bordered w-full max-w-full"
                onchange={(e) => {
                    onImageChange(e.currentTarget.files);
                }}
                type="file"
                capture
            />

            <div class="modal-action flex justify-between w-full">
                <button
                    class="btn btn-error"
                    onclick={() => {
                        modalState.existing?.beverage_id &&
                            $deleteMutation.mutate(
                                modalState.existing.beverage_id,
                            );
                    }}
                    disabled={!modalState.existing}>Delete</button
                >
                <button type="submit" class="btn btn-primary">Save</button>
            </div>
        </form>
    </div>
</div>
