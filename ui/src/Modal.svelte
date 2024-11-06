<script lang="ts">
    import { getKinds, getProducers, addBeverage } from "./api/client";

    interface Props {
        close: () => void;
    }

    let props: Props = $props();

    let kind = $state('');
    let producer = $state('');
    let name = $state('');

    const submit = () => {
        addBeverage(name, kind, producer)
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

        <select class="select select-bordered w-full max-w-xs" bind:value={kind}>
            {#await getKinds() then kinds}
                {#each kinds ?? [] as kind}
                    <option value={kind.kind_id}>{kind.name}</option>
                {/each}
            {/await}
        </select>

        <select class="select select-bordered w-full max-w-xs" bind:value={producer}>
            {#await getProducers() then producers}
                {#each producers ?? [] as producer}
                    <option value={producer.producer_id}>{producer.name}</option>
                {/each}
            {/await}
        </select>
        <button class="btn" onclick={submit}>Button</button>
    </div>
</div>
