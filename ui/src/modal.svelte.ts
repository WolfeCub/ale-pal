import type { UpdateBeverageRequest } from "./api/rspc"

interface ModalState {
    show: boolean;
    existing: UpdateBeverageRequest | null;
}

let state: ModalState = $state({
    show: false,
    existing: null,
})

export const modalState = {
    get show() {
        return state.show;
    },
    get existing() {
        return state.existing;
    },
    open(newVal?: UpdateBeverageRequest) {
        state.existing = newVal ?? null;
        state.show = true;
    },
    close() {
        state.show = false;
        state.existing = null;
    },
}

