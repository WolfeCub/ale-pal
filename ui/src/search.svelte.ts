import type { SearchBeveragesRequest } from "./api/rspc";

let state: SearchBeveragesRequest = $state({
    query: '',
})

export const searchState = {
    get query() {
        return state.query;
    },
    set query(text: string) {
        state.query = text;
    },
}


