import { createMutation, createQuery, QueryClient } from '@tanstack/svelte-query';

import { createClient, FetchTransport } from "@rspc/client";
import type { InsertBeverage, JoinBeverage, Kind, Procedures, Producer } from "./rspc";

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


export const useKindsQuery = () => createQuery<Kind[]>({
    queryKey: ['kinds'],
    queryFn: () => client.query(['kind']),
});

export const useProducersQuery = () => createQuery<Producer[]>({
    queryKey: ['producers'],
    queryFn: () => client.query(['producer']),
});

export const useBeveragesQuery = () => createQuery<JoinBeverage[]>({
    queryKey: ['beverage'],
    queryFn: () => client.query(['beverage']),
});

export const useBeverageMutation = () => createMutation({
    mutationFn: (beverage: InsertBeverage) => client.mutation(['beverage', beverage]),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ['beverage'] }), // TODO: Set the data rather than refetching
});

