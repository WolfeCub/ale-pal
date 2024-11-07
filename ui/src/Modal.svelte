<script lang="ts">
    import { getKinds, getProducers, addBeverage } from "./api/client";

    interface Props {
        close: () => void;
    }

    let props: Props = $props();

    let kind = $state("");
    let producer = $state("");
    let name = $state("");
    let rating = $state(10);
    let description = $state("");

    const submit = () => {
        addBeverage({
            name: name,
            producer_id: producer,
            kind_id: kind,
            rating: rating,
            description: description,
        });
    };

    const range = (start: number, end: number, length = end - start + 1) => Array.from({ length }, (_, i) => start + i);

    const onChange = (value: number) => {
        rating = value;
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

        <select
            class="select select-bordered w-full max-w-xs"
            bind:value={kind}
        >
            {#await getKinds() then kinds}
                {#each kinds ?? [] as kind}
                    <option value={kind.kind_id}>{kind.name}</option>
                {/each}
            {/await}
        </select>

        <select
            class="select select-bordered w-full max-w-xs"
            bind:value={producer}
        >
            {#await getProducers() then producers}
                {#each producers ?? [] as producer}
                    <option value={producer.producer_id}>{producer.name}</option
                    >
                {/each}
            {/await}
        </select>

        <textarea
            class="textarea textarea-bordered"
            placeholder="Description"
            bind:value={description}
        ></textarea>

        <div class="rating rating-lg rating-half">
            <input type="radio" name="rating-10" class="rating-hidden" onchange={() => onChange(0)}/>
            {#each range(0, 9) as num}
                <input
                    type="radio"
                    name="rating-10"
                    class="mask mask-star-2 mask-half-1 bg-green-500"
                    onchange={() => onChange(num + 0.5)}
                />
                <input
                    type="radio"
                    name="rating-10"
                    class="mask mask-star-2 mask-half-2 bg-green-500"
                    onchange={() => onChange(num + 1)}
                />
            {/each}
        </div>

        <button class="btn" onclick={submit}>Button</button>
    </div>
</div>
