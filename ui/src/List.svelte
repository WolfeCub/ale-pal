<script lang="ts">
    import { useBeveragesQuery } from "./api/client";

    const beveragesQuery = useBeveragesQuery();
</script>

{#if $beveragesQuery.isSuccess}
    {#each $beveragesQuery.data ?? [] as b}
        <div class="card bg-base-100 w-96 shadow-xl mb-3">
            <div class="card-body">
                <h2 class="card-title">
                    {b.name}
                    <div class="badge badge-secondary">
                        {b.rating}/10
                    </div>
                </h2>
                <p>{b.description}</p>
                <div class="card-actions justify-end">
                    <div class="badge badge-outline">{b.producer}</div>
                    <div class="badge badge-outline">{b.kind}</div>
                </div>
            </div>
        </div>
    {/each}
{/if}
