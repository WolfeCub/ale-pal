import axios from 'axios';

const client = axios.create({
    baseURL: 'http://localhost:8080',
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

export async function addBeverage(name: string, kind_id: string, producer_id: string): Promise<void> {
    let blah = { name, kind_id: kind_id, producer_id: producer_id };
    console.log(blah);
    await client.post(`/beverage`, blah).catch(e => { console.error(e); return null; });
}
