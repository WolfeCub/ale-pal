import type { SearchBeveragesRequest, SortOptions } from "./api/rspc";

const defaultSort: SortOptions = {
    column: 'beverage_id',
    direction: "asc",
};

let state: SearchBeveragesRequest = $state({
    query: '',
    sort: defaultSort,
})

export const searchState = {
    get value() {
        return state;
    },
    get query() {
        return state.query;
    },
    set query(text: string) {
        state.query = text;
    },
    get sort(): SortOptions {
        return state.sort;
    },
    set sort(sort: SortOptions) {
        state.sort = sort;
    },
    resetQuery() {
        state.query = '';
    },
    resetSort() {
        state.sort = defaultSort;
    },
}


