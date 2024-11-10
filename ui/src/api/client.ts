import { createMutation, createQuery, QueryClient } from '@tanstack/svelte-query';

import { createClient, FetchTransport } from "@rspc/client";
import type { InsertBeverage, JoinBeverage, Kind, Procedures, Producer, UpdateBeverageRequest } from "./rspc";

const client = createClient<Procedures>({
    transport: new FetchTransport("http://localhost:8080/rspc"),
});

export const queryClient = new QueryClient({
    defaultOptions: {
        queries: {
            staleTime: Infinity,
        },
    },
});


export const getKindsQuery = () => createQuery<Kind[]>({
    queryKey: ['kinds'],
    queryFn: () => client.query(['kind']),
});

export const getProducersQuery = () => createQuery<Producer[]>({
    queryKey: ['producers'],
    queryFn: () => client.query(['producer']),
});

export const getBeveragesQuery = () => createQuery<JoinBeverage[]>({
    queryKey: ['beverage'],
    queryFn: () => client.query(['beverage']),
});

export const upsertBeverageMutation = () => createMutation({
    mutationFn: (updateRequest: UpdateBeverageRequest) => client.mutation(['beverage', updateRequest]),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ['beverage'] }), // TODO: Set the data rather than refetching
});

export const deleteBeverageMutation = () => createMutation({
    mutationFn: (beverageId: number) => client.mutation(['deleteBeverage', beverageId]),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ['beverage'] }), // TODO: Set the data rather than refetching
});

