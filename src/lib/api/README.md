# API Layer

TypeScript wrappers around Tauri IPC commands.

## Purpose

- Provide type-safe API for invoking Tauri commands
- Handle error translation from Rust to TypeScript
- Centralize all backend communication

## Examples

```typescript
// src/lib/api/call.ts
import { invoke } from '@tauri-apps/api/tauri';

export async function initiateCall(number: string): Promise<string> {
  return await invoke('initiate_call', { number });
}
```

## Structure

```
api/
├── call.ts       # Call-related commands
├── auth.ts       # Authentication commands
├── audio.ts      # Audio device commands
└── settings.ts   # Settings commands
```
