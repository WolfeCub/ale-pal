<script lang="ts">
    import {
        deleteKindMutation,
        deleteProducerMutation,
        getKindsQuery,
        getProducersQuery,
    } from "./api/client";

    const kindsQuery = getKindsQuery();
    const producersQuery = getProducersQuery();
    const deleteKind = deleteKindMutation();
    const deleteProducer = deleteProducerMutation();
</script>

<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<div class="container mx-auto p-6 space-y-8">
    <div class="collapse bg-base-200">
        <input type="checkbox" />
        <div class="collapse-title text-xl font-medium">Kinds</div>
        <div class="collapse-content">
            <ul class="space-y-4">
                {#if $kindsQuery.isSuccess}
                    {#each $kindsQuery.data as kind}
                        <li class="flex justify-between items-center">
                            <span>{kind.name}</span>
                            <button
                                class="btn btn-sm btn-error"
                                on:click={(e) => {
                                    $deleteKind.mutate(kind.kind_id);
                                }}
                            >
                                Delete
                            </button>
                        </li>
                    {/each}
                {/if}
            </ul>
        </div>
    </div>

    <div class="collapse bg-base-200">
        <input type="checkbox" />
        <div class="collapse-title text-xl font-medium">Producers</div>
        <div class="collapse-content">
            <ul class="space-y-4">
                {#if $producersQuery.isSuccess}
                    {#each $producersQuery.data as producer}
                        <li class="flex justify-between items-center">
                            <span>{producer.name}</span>
                            <button
                                class="btn btn-sm btn-error"
                                on:click={(e) => {
                                    $deleteProducer.mutate(
                                        producer.producer_id,
                                    );
                                }}
                            >
                                Delete
                            </button>
                        </li>
                    {/each}
                {/if}
            </ul>
        </div>
    </div>
</div>
