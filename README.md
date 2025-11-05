# RUSTALK

An open-source, white-label VoIP desktop application built with Rust, Tauri, and SvelteKit.

## Development

### Prerequisites

- Rust (latest stable)
- Node.js 20+
- npm or yarn

### Setup

1. Install dependencies:

   ```bash
   npm install
   ```

2. Run development server:
   ```bash
   npm run tauri:dev
   ```

### Build

Build the production app:

```bash
npm run tauri:build
```

### Testing

Run frontend tests:

```bash
npm test
```

Run Rust tests:

```bash
cd src-tauri
cargo test
```

## Project Structure

```
rustalk/
├── src/                    # SvelteKit frontend
│   ├── routes/            # SvelteKit routes
│   └── lib/               # Shared libraries
├── src-tauri/              # Tauri backend
│   ├── src/
│   │   ├── commands/      # Tauri command handlers
│   │   └── main.rs        # Tauri entry point
│   └── Cargo.toml         # Rust dependencies
└── package.json           # Node.js dependencies
```

## Architecture

See [docs/architecture/](docs/architecture/) for detailed architecture documentation.

## License

Apache 2.0
