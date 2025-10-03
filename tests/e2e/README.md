# E2E Tests

Playwright end-to-end tests for critical user paths.

## Purpose

- Test complete user workflows
- Verify cross-platform compatibility
- Catch integration issues

## Structure

```
e2e/
├── call-flow.spec.ts       # Outbound/inbound call tests
├── auth.spec.ts            # Registration tests
├── audio-devices.spec.ts   # Audio device selection
└── settings.spec.ts        # Settings persistence
```

## Running

```bash
npm run test:e2e
```
