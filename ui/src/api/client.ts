import axios from 'axios';

const client = axios.create({
    baseURL: 'http://localhost:8080',
    headers: {
        'Content-Type': 'application/json',
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

export async function getKinds(): Promise<Kind[] | null> {
    return await client.get<Kind[]>('/kind')
        .then(r => r.data)
        .catch(e => { console.error(e); return null; });
}

export async function getProducers(): Promise<Producer[] | null> {
    return await client.get<Producer[]>('/producer')
        .then(r => r.data)
        .catch(e => { console.error(e); return null; });
}

export async function getBeverages(): Promise<Beverage[] | null> {
    return await client.get<Beverage[]>('/beverage')
        .then(r => r.data)
        .catch(e => { console.error(e); return null; });
}

export async function addKind(name: string): Promise<void> {
    await client.post(`/kind/${name}`)
        .catch(e => { console.error(e); return null; });
}

export async function addProducer(name: string): Promise<void> {
    await client.post(`/producer/${name}`)
        .catch(e => { console.error(e); return null; });
}

export async function addBeverage(beverage: InsertBeverage): Promise<void> {
    await client.post(`/beverage`, beverage).catch(e => { console.error(e); return null; });
}
