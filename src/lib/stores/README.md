# Stores

Svelte stores for reactive state management.

## Purpose

- Manage application state
- React to backend events
- Provide reactive UI updates

## Examples

```typescript
// src/lib/stores/call.ts
import { writable } from 'svelte/store';

export const activeCall = writable<Call | null>(null);
```

## Structure

```
stores/
├── call.ts       # Call state
├── auth.ts       # Authentication state
├── audio.ts      # Audio device state
└── settings.ts   # User settings
```
