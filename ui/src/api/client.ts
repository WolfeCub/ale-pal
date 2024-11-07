import { createMutation, createQuery, QueryClient } from '@tanstack/svelte-query';
import axios from 'axios';

const client = axios.create({
    baseURL: 'http://localhost:8080',
    headers: {
        'Content-Type': 'application/json',
    },
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

interface InsertBeverage {
    name: string;
    producer_id: string;
    kind_id: string;
    rating: number;
    description: string;
}

export const useKindsQuery = () => createQuery<Kind[]>({
    queryKey: ['kinds'],
    queryFn: () => client.get<Kind[]>('/kind').then(o => o.data),
});

export const useProducersQuery = () => createQuery<Producer[]>({
    queryKey: ['producers'],
    queryFn: () => client.get<Producer[]>('/producer').then(o => o.data),
});

export const useBeveragesQuery = () => createQuery<Beverage[]>({
    queryKey: ['beverage'],
    queryFn: () => client.get<Beverage[]>('/beverage').then(o => o.data),
});

export const useBeverageMutation = () => createMutation({
    mutationFn: (beverage: InsertBeverage) => client.post(`/beverage`, beverage),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ['beverage'] }), // TODO: Set the data rather than refetching
});

export async function addKind(name: string): Promise<void> {
    await client.post(`/kind/${name}`)
        .catch(e => { console.error(e); return null; });
}

export async function addProducer(name: string): Promise<void> {
    await client.post(`/producer/${name}`)
        .catch(e => { console.error(e); return null; });
}
