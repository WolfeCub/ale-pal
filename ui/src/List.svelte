<script lang="ts">
    import { getBeveragesQuery } from "./api/client";
    import { byteArrayToBlob } from "./utils";
    import { modalState } from "./modal.svelte";
    import { searchState } from "./search.svelte";

    const beveragesQuery = $derived(getBeveragesQuery({ query: searchState.query }));
</script>

{#if $beveragesQuery.isSuccess}
    {#each $beveragesQuery.data ?? [] as b}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="card bg-base-100 w-96 shadow-xl mb-3" onclick={() => modalState.open({ beverage_id: b.beverage_id, beverage: b })}>
            <figure>
                <img
                    src={URL.createObjectURL(byteArrayToBlob(b.image!))}
                    alt=""
                />
            </figure>
            <div class="card-body">
                <div>
                    <h2 class="card-title">
                        {b.name}
                    </h2>
                    <p class="text-sm text-gray-600">Brewed by: {b.producer}</p>
                    <p class="text-sm text-gray-600">Type: {b.kind}</p>
                </div>

                <div class="flex items-center space-x-2">
                    <div class="rating rating-xs rating-half">
                        {#each Array(20) as _, i (i)}
                            <input
                                disabled
                                type="radio"
                                name={`rating-${b.name}`}
                                class={"mask mask-star-2 bg-orange-400 " +
                                    (i % 2 == 0
                                        ? "mask-half-1"
                                        : "mask-half-2")}
                                value={i / 2 + 0.5}
                                bind:group={b.rating}
                            />
                        {/each}
                    </div>
                    <div class="text-xs text-gray-600">({b.rating}/10)</div>
                </div>

                <p>{b.description}</p>
            </div>
        </div>
    {/each}
{/if}
