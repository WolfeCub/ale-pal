<script lang="ts">
    import { nav } from "./routing.svelte";
    import { searchState } from "./search.svelte";

    let showSearch = $state(false);

    let timer = $state(0);

    const debounce = (value: string) => {
        clearTimeout(timer);
        timer = setTimeout(() => {
            searchState.query = value;
        }, 500);
    };

    $effect(() => {
        if (!showSearch) {
            searchState.query = '';
        }
    })
</script>

<!-- svelte-ignore a11y_consider_explicit_label -->
<!-- svelte-ignore a11y_missing_attribute -->
<div class="navbar bg-base-100">
    <div class="navbar-start">
        <div class="dropdown">
            <div tabindex="0" role="button" class="btn btn-ghost btn-circle">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-5 w-5"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M4 6h16M4 12h16M4 18h7"
                    />
                </svg>
            </div>
            <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
            <ul
                tabindex="0"
                class="menu menu-sm dropdown-content bg-base-100 rounded-box z-[1] mt-3 w-52 p-2 shadow"
            >
                <li><a onclick={() => nav("/")}>Home</a></li>
                <li><a onclick={() => nav("/settings")}>Settings</a></li>
            </ul>
        </div>
    </div>
    {#if !showSearch}
        <div class="navbar-center">
            <img src="/logo-transparent.png" class="h-12" alt="logo" />
            <a class="btn btn-ghost text-xl">Ale Pal</a>
        </div>
    {:else}
        <input
            onkeyup={(e) => debounce(e.currentTarget.value)}
            type="text"
            placeholder="Search..."
            class="input input-bordered w-full max-w-xs"
        />
    {/if}
    <div class="navbar-end">
        <button
            class="btn btn-ghost btn-circle"
            onclick={() => {
                showSearch = !showSearch;
            }}
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-5 w-5"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                />
            </svg>
        </button>
    </div>
</div>
