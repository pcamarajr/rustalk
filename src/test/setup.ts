import { expect, afterEach, vi } from 'vitest';
import '@testing-library/jest-dom/vitest';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
	invoke: vi.fn()
}));

