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


const kindQueryKey = ['kinds'];
export const getKindsQuery = () => createQuery<Kind[]>({
    queryKey: kindQueryKey,
    queryFn: () => client.query(['kind']),
});

const producerQueryKey = ['producers'];
export const getProducersQuery = () => createQuery<Producer[]>({
    queryKey: producerQueryKey,
    queryFn: () => client.query(['producer']),
});

const beverageQueryKey = ['beverage'];
export const getBeveragesQuery = () => createQuery<JoinBeverage[]>({
    queryKey: beverageQueryKey,
    queryFn: () => client.query(['beverage']),
});

export const upsertBeverageMutation = () => createMutation({
    mutationFn: (updateRequest: UpdateBeverageRequest) => client.mutation(['beverage', updateRequest]),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: beverageQueryKey }), // TODO: Set the data rather than refetching
});

export const insertKindMutation = () => createMutation({
    mutationFn: (insertKind: NameRequest) => client.mutation(['kind', insertKind]),
    onSuccess: (kindId, nameRequest) => {
        queryClient.setQueryData(kindQueryKey, (oldData: Kind[]): Kind[] => [...oldData, { kind_id: kindId, name: nameRequest.name }])
    },
});

export const insertProducerMutation = () => createMutation({
    mutationFn: (insertProducer: NameRequest) => client.mutation(['producer', insertProducer]),
    onSuccess: (producerId, nameRequest) => {
        queryClient.setQueryData(producerQueryKey, (oldData: Producer[]): Producer[] => [...oldData, { producer_id: producerId, name: nameRequest.name }])
    },
});

export const deleteKindMutation = () => createMutation({
    mutationFn: (kindId: number) => client.mutation(['deleteKind', kindId]),
    onSuccess: (_, kindId) => {
        queryClient.setQueryData(kindQueryKey, (oldData: Kind[]): Kind[] => oldData.filter(k => k.kind_id != kindId))
    },
});

export const deleteProducerMutation = () => createMutation({
    mutationFn: (producerId: number) => client.mutation(['deleteProducer', producerId]),
    onSuccess: (_, producerId) => {
        queryClient.setQueryData(producerQueryKey, (oldData: Producer[]): Producer[] => oldData.filter(k => k.producer_id != producerId))
    },
});

export const deleteBeverageMutation = () => createMutation({
    mutationFn: (beverageId: number) => client.mutation(['deleteBeverage', beverageId]),
    onSuccess: (_, beverageId) => {
        queryClient.setQueryData(beverageQueryKey, (oldData: JoinBeverage[]): JoinBeverage[] => oldData.filter(k => k.beverage_id != beverageId))
    },
});

