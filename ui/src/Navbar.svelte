<script lang="ts">
    import type { SortColumn, SortDir, SortOptions } from "./api/rspc";
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
            searchState.resetQuery();
        }
    });

    const sortFields: { display: string; column: SortColumn }[] = [
        { display: "Date Added", column: "beverage_id" },
        { display: "Rating", column: "rating" },
        { display: "Name", column: "name" },
        { display: "Kind", column: "kind" },
    ];


    const changeSortField = (field: SortColumn) => {
        if (searchState.sort.column === field) {
            // Reset to default sort if the same field is clicked
            searchState.resetSort();
        } else {
            searchState.sort = { ...searchState.sort, column: field, };
        }
    };

    const changeSortDirection = (direction: SortDir) => {
        searchState.sort = { ...searchState.sort, direction };
    };
</script>

<!-- svelte-ignore a11y_consider_explicit_label -->
<!-- svelte-ignore a11y_missing_attribute -->
<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
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
            <a class="btn btn-ghost text-xl" onclick={() => nav("/")}>Ale Pal</a>
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

        <div class="dropdown dropdown-end">
            <div tabindex="0" role="button" class="btn btn-ghost btn-circle">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    class="size-6"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M3.75 6.75h16.5M3.75 12h16.5M12 17.25h8.25"
                    />
                </svg>
            </div>

            <ul
                tabindex="0"
                class="menu menu-sm dropdown-content bg-base-100 rounded-box z-[1] mt-3 w-52 p-2 shadow"
            >
                <li>
                    <span class="menu-title">Sort by:</span>
                    <ul class="menu menu-compact">
                        {#each sortFields as field}
                            <li>
                                <a
                                    class={searchState.sort.column === field.column
                                        ? "text-primary font-bold"
                                        : ""}
                                    onclick={() =>
                                        changeSortField(field.column)}
                                >
                                    {field.display}
                                </a>
                            </li>
                        {/each}
                    </ul>
                </li>

                <li>
                    <span class="menu-title">Direction:</span>
                    <ul class="menu menu-compact">
                        <li>
                            <a
                                class={searchState.sort.direction === "asc"
                                    ? "text-primary font-bold"
                                    : ""}
                                onclick={() => changeSortDirection("asc")}
                            >
                                Ascending
                            </a>
                        </li>
                        <li>
                            <a
                                class={searchState.sort.direction === "desc"
                                    ? "text-primary font-bold"
                                    : ""}
                                onclick={() => changeSortDirection("desc")}
                            >
                                Descending
                            </a>
                        </li>
                    </ul>
                </li>
            </ul>
        </div>
    </div>
</div>
