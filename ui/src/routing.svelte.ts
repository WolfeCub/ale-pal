import type { Component } from "svelte";

export const initializeRouting = () => {
    const originalPushState = window.history.pushState;

    const newPushState = (...args: [data: any, unused: string, url?: string | URL | null]) => {
        originalPushState.apply(window.history, args)
        window.dispatchEvent(new Event("pushState"));
    };

    window.history.pushState = newPushState;
}

export const createPathname = () => {
    let pathname = $state(window.location.pathname);

    window.addEventListener("pushState", () => {
        pathname = window.location.pathname;
    });

    return {
        get path() { return pathname },
    }
}

export interface Route {
    path: string;
    component: Component
}

export const nav = (path: string) => {
        history.pushState({}, null!, path);
};
