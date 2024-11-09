<script lang="ts">
    import { useBeveragesQuery } from "./api/client";

    const beveragesQuery = useBeveragesQuery();
</script>

{#if $beveragesQuery.isSuccess}
    {#each $beveragesQuery.data ?? [] as b}
        <div class="card bg-base-100 w-96 shadow-xl mb-3">
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
