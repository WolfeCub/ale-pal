interface ToastState {
    show: boolean;
    text: string;
    time: number;
}

const defaultText = 'Something went wrong';
const defaultTime = 3000;

let state: ToastState = $state({
    show: false,
    text: defaultText,
    time: defaultTime,
})

export const toastState = {
    get show() {
        return state.show;
    },
    get text() {
        return state.text;
    },
    get time() {
        return state.time;
    },
    open(text?: string, time?: number) {
        state.show = true;
        state.text = text ?? defaultText;
        state.time = time ?? defaultTime;
    },
    close() {
        state.show = false;
    },
}

