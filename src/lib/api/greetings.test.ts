import { describe, it, expect, vi, beforeEach } from 'vitest';
import { greet } from './greetings';
import * as tauriApi from '@tauri-apps/api/tauri';

// Mock Tauri API
vi.mock('@tauri-apps/api/tauri', () => ({
	invoke: vi.fn()
}));

describe('greetings', () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	it('should call Tauri invoke with correct parameters', async () => {
		const mockInvoke = vi.mocked(tauriApi.invoke);
		mockInvoke.mockResolvedValue('Hello, Test! You\'ve been greeted from Rust!');

		const result = await greet('Test');

		expect(mockInvoke).toHaveBeenCalledWith('greet', { name: 'Test' });
		expect(result).toBe('Hello, Test! You\'ve been greeted from Rust!');
	});
});

