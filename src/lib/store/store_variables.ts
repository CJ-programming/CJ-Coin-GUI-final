import { writable } from "svelte/store";

export const seed_node_ip = ''
export const filePath = writable<string>('');
export const password = writable<string>('');

async function getData(url: string): Promise<any> {
	const response = await fetch(url);
	if (!response.ok) throw new Error("Failed to fetch");
	return response.json();
}

async function postData(url: string, data: any): Promise<any> {
	const response = await fetch(url, {
		method: "POST",
		headers: {
		"Content-Type": "application/json"
		},
		body: JSON.stringify(data)
	});
	if (!response.ok) throw new Error("Failed to post");
	return response.json();
}