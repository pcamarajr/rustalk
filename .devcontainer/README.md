# Rustalk Development Container

This directory contains configuration for the Rustalk development container, which provides a consistent, reproducible development environment for all contributors.

## Architecture Decisions

### Base Image: Debian 12 (bookworm)

**Why Debian?**

- Stable, well-maintained base image
- Excellent Rust toolchain support
- Wide availability of development packages
- Smaller image size compared to Ubuntu
- Good balance of stability and package availability

**Alternative considered**: Ubuntu 24.04 LTS

- Chosen Debian for smaller footprint and excellent Rust ecosystem support

### Rust Installation: rustup

**Why rustup?**

- Official Rust toolchain installer
- Easy toolchain management (stable, beta, nightly)
- Component management (clippy, rustfmt, etc.)
- Standard in Rust community

### Node.js Installation: NodeSource Repository

**Why NodeSource over Debian packages?**

- Latest Node.js 20 LTS with security updates
- Better npm compatibility
- Official Node.js distribution
- Aligns with project requirement for Node.js 20

### Tauri Dependencies

The container includes Linux Tauri dependencies (`libwebkit2gtk`, `libgtk-3-dev`, etc.), but:

**Important Note**: Since we're building for macOS/Windows first:

- Container is primarily for **development** (code editing, testing)
- Actual **builds** for macOS/Windows use host system libraries
- Linux dependencies are included for completeness and future Linux support

### Non-Root User: `vscode`

**Why non-root?**

- Security best practice
- Matches VS Code's default devcontainer user
- Prevents accidental system modification
- Better file permission handling

## Container Components

### Dockerfile Structure

```dockerfile
1. Base image + system tools
2. Node.js 20 LTS installation
3. Rust toolchain installation
4. Tauri system dependencies (Linux)
5. Rust development tools (cargo-nextest, etc.)
6. Tauri CLI installation
7. Non-root user setup
```

### Port Forwarding

The container forwards these ports:

- **1420**: Tauri development server
- **5173**: Vite HMR (SvelteKit frontend hot reload)

Configured in `devcontainer.json` with auto-forward and notification.

### Mount Points

- **Workspace**: Project directory mounted at `/workspace`
- **Consistency**: `cached` mode for better macOS performance

## Customization Guide

### Adding More Rust Tools

Edit `.devcontainer/Dockerfile` and add to the cargo install section:

```dockerfile
RUN cargo install cargo-nextest cargo-llvm-cov cargo-audit <new-tool> --locked
```

Then rebuild the container.

### Adding System Packages

Edit `.devcontainer/Dockerfile` and add to appropriate `apt-get install` section:

```dockerfile
RUN apt-get update && apt-get install -y \
    existing-package \
    new-package \
    && rm -rf /var/lib/apt/lists/*
```

### Changing Node.js Version

Edit `.devcontainer/Dockerfile` and modify the NodeSource setup:

```dockerfile
RUN curl -fsSL https://deb.nodesource.com/setup_<VERSION>.x | bash -
```

### Adding VS Code Extensions

Edit `.devcontainer/devcontainer.json` and add to `customizations.vscode.extensions`:

```json
"extensions": [
    "existing.extension",
    "new.extension-id"
]
```

### Modifying VS Code Settings

Edit `.devcontainer/devcontainer.json` and modify `customizations.vscode.settings`.

Note: These settings apply only inside the container. Local `.vscode/settings.json` applies when not using container.

## Performance Considerations

### macOS File System Performance

Docker on macOS uses a VM layer, which can make file I/O slower than native. We mitigate this by:

1. **Excluding build artifacts** from file watching:

   ```json
   "files.watcherExclude": {
       "**/target/**": true,
       "**/node_modules/**": true
   }
   ```

2. **Using cached mount consistency** for workspace

3. **Future optimization**: Consider volume mounts for `target/` and `node_modules/` for better performance

### Build Artifacts

- `target/` (Rust build artifacts): Can be large, excluded from watching
- `node_modules/`: Excluded from watching
- These can be regenerated, so safe to exclude

## Container Lifecycle

### Building

Container builds automatically when:

1. Opening project for first time
2. After modifying Dockerfile
3. Manual rebuild via `F1` → "Remote-Containers: Rebuild Container"

### Starting

Container starts when:

1. Opening project in VS Code/Cursor
2. Container automatically starts if configured in `devcontainer.json`

### Stopping

Container stops when:

1. Closing VS Code/Cursor
2. Manual stop: `F1` → "Remote-Containers: Stop Container"

### Cleaning Up

To free disk space:

```bash
# Remove unused containers
docker container prune

# Remove unused images
docker image prune

# Full cleanup (be careful!)
docker system prune -a
```

## Troubleshooting Container Issues

### Container Build Fails

1. Check Docker Desktop is running
2. Verify enough disk space
3. Check Docker logs: `docker logs <container-id>`
4. Try manual build: `docker build -t rustalk-dev .devcontainer/`

### Tools Missing After Rebuild

1. Check `setup.sh` ran successfully
2. Verify Dockerfile install commands
3. Check container logs for errors

### Port Conflicts

Edit `devcontainer.json` and change `forwardPorts` if conflicts occur.

### Permission Issues

If you see permission errors:

1. Verify user is `vscode` (non-root)
2. Check file ownership in container
3. May need to fix permissions on mounted volume

## Security Considerations

- **Non-root user**: Container runs as `vscode` user
- **Minimal base image**: Debian slim reduces attack surface
- **Locked cargo tools**: Using `--locked` flag for reproducible installs
- **No secrets in container**: Credentials managed via host system (Keychain/Credential Manager)

## Future Enhancements

Potential improvements for future iterations:

1. **Multi-stage builds**: Separate builder and runtime images
2. **Volume mounts**: Better performance for `target/` and `node_modules/`
3. **Docker Compose**: For complex multi-service setups
4. **Cached layers**: Pre-built base images for faster builds
5. **Build container**: Separate container specifically for CI/CD builds

## References

- [Dev Containers Documentation](https://containers.dev/)
- [VS Code Remote Development](https://code.visualstudio.com/docs/remote/remote-overview)
- [Tauri Development Guide](https://tauri.app/v1/guides/development/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [SvelteKit Documentation](https://kit.svelte.dev/)

---

**Maintained by**: Rustalk Development Team  
**Last Updated**: 2025-01-XX


