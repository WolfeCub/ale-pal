<script lang="ts">
    import { getBeverages } from "./api/client";
</script>

{#await getBeverages()}
    Loading ...
{:then beverages}
    {#each beverages ?? [] as b}
        <div class="card bg-base-100 w-96 shadow-xl">
            <div class="card-body">
                <h2 class="card-title">
                    {b.name}
                    <div class="badge badge-secondary">
                        {b.producer}
                    </div>
                </h2>
                <p>If a dog chews shoes whose shoes does he choose?</p>
                <div class="card-actions justify-end">
                    <div class="badge badge-outline">{b.kind}</div>
                </div>
            </div>
        </div>
    {/each}
{/await}
