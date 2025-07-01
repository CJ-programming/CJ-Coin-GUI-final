import { invoke } from '@tauri-apps/api/tauri';

export async function generate_key_pair(): Promise<[string, string]> {
    console.log("Key pair has been called");
    return invoke('generate_key_pair');
}

export async function sign(privateKeyHex: string, message: string): Promise<[string, string]> {
    return invoke('sign', { privateKeyHex : privateKeyHex, message : message });
}

export async function verify() {
	return 0;
}