<script lang="ts">
    import { getKinds, getProducers, addKind, addProducer, getBeverages, addBeverage } from "./api/client";

    const promise = Promise.all([getKinds(), getProducers(), getBeverages()]);

    let kind = $state('');
    let producer = $state('');
    let name = $state('');
</script>

<main>
    <h1>Ale Pal</h1>

    <div>
        {#await promise}
            Loading ...
        {:then [kinds, producers, beverages]}
            <select bind:value={kind} placeholder="kind">
                {#each kinds ?? [] as kind}
                    <option value={kind.kind_id}>{kind.name}</option>
                {/each}
            </select>
            <select bind:value={producer} placeholder="producer">
                {#each producers ?? [] as producer}
                    <option value={producer.producer_id}>{producer.name}</option>
                {/each}
            </select>
            <input bind:value={name} placeholder="name" />
            <button onclick={() => addBeverage(name, kind, producer)}>Submit</button>

            <h3>Drinks</h3>
            <ul>
                {#each beverages ?? [] as b}
                    <li>{b.name} - {b.producer} - {b.kind}</li>
                {/each}
            </ul>
        {/await}
    </div>

</main>
