import { createMutation, createQuery, QueryClient } from '@tanstack/svelte-query';

import { createClient, FetchTransport } from "@rspc/client";
import type { InsertBeverage, Procedures } from "./rspc";

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

interface Kind {
    kind_id: number;
    name: string;
}

interface Producer {
    producer_id: number;
    name: string;
}

interface Beverage {
    name: string;
    producer: string;
    kind: string;
    rating: number;
    description: string;
}

// interface InsertBeverage {
//     name: string;
//     producer_id: string;
//     kind_id: string;
//     rating: number;
//     description: string;
// }

// export const useKindsQuery = () => createQuery<Kind[]>({
//     queryKey: ['kinds'],
//     queryFn: () => client.get<Kind[]>('/kind').then(o => o.data),
// });

export const useKindsQuery = () => createQuery<Kind[]>({
    queryKey: ['kinds'],
    queryFn: () => client.query(['kind']),
});

export const useProducersQuery = () => createQuery<Producer[]>({
    queryKey: ['producers'],
    queryFn: () => client.query(['producer']),
});

export const useBeveragesQuery = () => createQuery<Beverage[]>({
    queryKey: ['beverage'],
    queryFn: () => client.query(['beverage']),
});

export const useBeverageMutation = () => createMutation({
    mutationFn: (beverage: InsertBeverage) => client.mutation(['beverage', beverage]),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ['beverage'] }), // TODO: Set the data rather than refetching
});

