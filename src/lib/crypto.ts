import { invoke } from '@tauri-apps/api/tauri';

export async function decrypt_file(aes_key_data_path: string, key_pair_data_path: string, password: string): Promise<string> {
	console.log("Decrypting file...")
	return invoke('decrypt_file', {aes_key_data_path: aes_key_data_path, key_pair_data_path: key_pair_data_path, password: password})
} 

export async function generate_aes_key(path: string, password: string, iterations: number) {
    console.log("Key pair has been called");
    return invoke('generate_key_pair', {path: path, password: password, iterations: iterations});
}

export async function generate_key_pair(path: string, hex_key: string): Promise<[string, string]> {
	console.log("Key pair has been generated")
	return invoke('generate_key_pair_to_file', {path : path, hex_key : hex_key })
}

export async function sign(privateKeyHex: string, message: string): Promise<[string, string]> {
    return invoke('sign', { privateKeyHex : privateKeyHex, message : message });
}