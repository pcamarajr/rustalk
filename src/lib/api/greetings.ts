import { invoke } from '@tauri-apps/api/tauri';

/**
 * Greet a person by name
 * @param name The name to greet
 * @returns A greeting message from the Rust backend
 */
export async function greet(name: string): Promise<string> {
	return await invoke<string>('greet', { name });
}

