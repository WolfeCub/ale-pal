import { createMutation, createQuery, QueryClient } from '@tanstack/svelte-query';

import { createClient, FetchTransport } from "@rspc/client";
import type { InsertBeverage, JoinBeverage, Kind, NameRequest, Procedures, Producer, UpdateBeverageRequest } from "./rspc";

const url = import.meta.env.MODE == 'production' ? '/rspc' : 'http://localhost:8080/rspc';

const client = createClient<Procedures>({
    transport: new FetchTransport(url),
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

export const insertKindMutation = () => createMutation({
    mutationFn: (insertKind: NameRequest) => client.mutation(['kind', insertKind]),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ['kinds'] }), // TODO: Set the data rather than refetching
});

export const insertProducerMutation = () => createMutation({
    mutationFn: (insertProducer: NameRequest) => client.mutation(['producer', insertProducer]),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ['producers'] }), // TODO: Set the data rather than refetching
});
