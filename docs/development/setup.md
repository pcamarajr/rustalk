# Development Environment Setup

This guide will help you set up the Rustalk development environment using a Docker devcontainer. This approach ensures a consistent, reproducible development environment across all developers and operating systems.

## Prerequisites

Before you begin, ensure you have the following installed on your machine:

- **Docker Desktop** (or Docker Engine + Docker Compose)
  - Download from: https://www.docker.com/products/docker-desktop
  - Verify installation: `docker --version`
- **VS Code** or **Cursor** with the **Remote - Containers** extension
  - VS Code: Install from Extensions marketplace (`ms-vscode-remote.remote-containers`)
  - Cursor: Built-in support for devcontainers

## Quick Start

1. **Clone the repository** (if you haven't already):
   ```bash
   git clone <repository-url>
   cd rustalk
   ```

2. **Open in VS Code/Cursor**:
   - Open VS Code/Cursor
   - Open the `rustalk` folder
   - When prompted, click "Reopen in Container" (or press `F1` → "Remote-Containers: Reopen in Container")

3. **Wait for container to build**:
   - The first time, Docker will build the container image (this takes ~5-10 minutes)
   - Subsequent starts are much faster (~30 seconds)
   - The setup script will run automatically and verify all tools

4. **Verify installation**:
   Open a terminal in VS Code/Cursor and run:
   ```bash
   rustc --version
   node --version
   cargo --version
   ```

## What's Included

The development container provides:

- **Rust**: Latest stable toolchain via rustup
- **Node.js**: Version 20 LTS
- **Tauri CLI**: Global installation via npm
- **Rust Tools**:
  - `cargo-nextest` - Next-generation test runner
  - `cargo-llvm-cov` - Code coverage tool
  - `cargo-audit` - Security vulnerability scanner
- **System Dependencies**: All required libraries for Tauri development
- **VS Code Extensions**: Automatically installed (rust-analyzer, Tauri, Svelte, ESLint, Prettier, TailwindCSS)

## Development Workflow

### Starting the Development Server

Once the container is running and the project is initialized (Phase 1), you can start the Tauri development server:

```bash
cargo tauri dev
```

This will:
- Build the Rust backend
- Start the SvelteKit frontend dev server
- Open the Tauri application window on your macOS

**Note**: The GUI window will display on your host Mac, not inside the container. This is expected behavior for Tauri development.

### Running Tests

**Rust tests**:
```bash
# Standard cargo test
cargo test

# Using cargo-nextest (recommended)
cargo nextest run

# With coverage
cargo llvm-cov --html
```

**Frontend tests** (after Phase 1 initialization):
```bash
npm test
```

### Building for Production

```bash
cargo tauri build
```

This creates platform-specific binaries in `src-tauri/target/release/`.

## Troubleshooting

### Container Won't Start

**Problem**: Container fails to build or start.

**Solutions**:
1. Ensure Docker Desktop is running
2. Check Docker has enough resources (minimum 4GB RAM recommended)
3. Try rebuilding: `F1` → "Remote-Containers: Rebuild Container"
4. Check Docker logs: `docker logs <container-id>`

### Tools Not Found

**Problem**: Commands like `rustc` or `node` not found.

**Solutions**:
1. Ensure you're in the container terminal (not local terminal)
2. Check terminal shows `@` symbol indicating container connection
3. Restart container: `F1` → "Remote-Containers: Rebuild Container"

### Port Already in Use

**Problem**: Ports 1420 or 5173 already in use.

**Solutions**:
1. Close other applications using these ports
2. Or modify `.devcontainer/devcontainer.json` to use different ports

### Slow File Performance on macOS

**Problem**: File operations are slow (common on macOS with Docker).

**Solutions**:
1. This is expected due to macOS file system performance with Docker
2. Consider excluding `target/` and `node_modules/` from file watching (already configured)
3. Future optimization: Use volume mounts for build artifacts

### Tauri CLI Not Found

**Problem**: `cargo tauri --version` fails.

**Solutions**:
1. This is normal before Phase 1 (project not initialized yet)
2. After Phase 1, Tauri CLI will be available via npm scripts
3. Or install globally: `npm install -g @tauri-apps/cli`

### GUI Window Doesn't Appear

**Problem**: `cargo tauri dev` runs but no window opens.

**Solutions**:
1. Check that you're running on the host Mac (not inside container GUI)
2. Verify X11/display forwarding (not needed for macOS - should work automatically)
3. Check Tauri configuration in `src-tauri/tauri.conf.json`

## Rebuilding the Container

If you need to rebuild the container (e.g., after modifying the Dockerfile):

1. Open command palette: `F1`
2. Run: "Remote-Containers: Rebuild Container"
3. Wait for rebuild (~5-10 minutes first time)

## Customizing the Container

To add more tools or dependencies:

1. **Add to Dockerfile**: Edit `.devcontainer/Dockerfile`
2. **Add VS Code extensions**: Edit `.devcontainer/devcontainer.json` → `customizations.vscode.extensions`
3. **Rebuild container**: `F1` → "Remote-Containers: Rebuild Container"

## Performance Tips

- **Exclude build artifacts**: `target/` and `node_modules/` are excluded from file watching
- **Use terminal in container**: All development should happen in container terminals
- **Disk space**: Docker images can be large; periodically clean unused images: `docker system prune`

## Next Steps

After the container is set up:

1. **Phase 1**: Initialize Tauri + SvelteKit project structure
2. **Phase 2**: Build UI foundation with design system
3. See [05-implementation-roadmap.md](../architecture/05-implementation-roadmap.md) for full development plan

## Getting Help

If you encounter issues not covered here:

1. Check container logs: View → Output → Select "Remote-Containers"
2. Review `.devcontainer/README.md` for technical details
3. Check architecture documentation in `docs/architecture/`

---

**Last Updated**: 2025-01-XX  
**Container Version**: Debian 12 (bookworm)  
**Rust Version**: Latest stable  
**Node.js Version**: 20 LTS



