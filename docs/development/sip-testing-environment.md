# Local SIP Testing Environment

This guide explains how to set up and use the local Asterisk SIP server for testing SIP integration in Rustalk.

## Overview

The local SIP testing environment uses Docker Compose to run an Asterisk server in a container. This provides a consistent, isolated testing environment that:

- Starts automatically with `npm run tauri:dev`
- Persists configuration across restarts
- Supports UDP and TCP SIP transports (TLS disabled - requires certificate configuration)
- Includes a pre-configured test user for integration testing

## Prerequisites

- **Docker Desktop** (or Docker Engine + Docker Compose)
  - Download from: https://www.docker.com/products/docker-desktop
  - Verify installation: `docker --version` and `docker-compose --version`

## Quick Start

### Starting the Environment

The Asterisk server starts automatically when you run:

```bash
npm run tauri:dev
```

This command:
1. Starts the Docker container with `docker-compose up -d`
2. Launches the Tauri development server

### Manual Control

You can also control the Docker containers manually:

```bash
# Start Asterisk server
npm run docker:start

# Stop Asterisk server
npm run docker:stop

# Stop Asterisk and Tauri dev server
npm run tauri:dev:stop
```

## Configuration

### Test User Credentials

A test SIP user is pre-configured for development and testing:

- **Username**: `testuser`
- **Password**: `testpass`
- **Domain**: `localhost` (or `127.0.0.1`)

These credentials match the configuration in `asterisk-config/pjsip.conf` and can be used for:
- Integration tests
- Manual SIP registration testing
- Call flow testing

**⚠️ Security Note**: These credentials are for local development only. Never use them in production or expose them publicly.

### SIP Server Endpoints

The Asterisk server exposes the following ports:

- **SIP UDP**: `localhost:5060`
- **SIP TCP**: `localhost:5060`
- **RTP**: `10000-10029/udp` (30 ports for local testing)

### Environment Variables

You can customize the SIP server configuration using environment variables. Copy `.env.example` to `.env` and modify as needed:

```bash
cp .env.example .env
```

Available variables:

- `SIP_SERVER_HOST` - SIP server hostname (default: `localhost`)
- `SIP_SERVER_PORT_UDP` - UDP port (default: `5060`)
- `SIP_SERVER_PORT_TCP` - TCP port (default: `5060`)
- `SIP_SERVER_PORT_TLS` - TLS port (default: `5061`)
- `SKIP_SIP_INTEGRATION_TESTS` - Set to `true` to skip SIP tests (useful for CI)
- `SIP_TEST_USER` - Test username (default: `testuser`)
- `SIP_TEST_PASSWORD` - Test password (default: `testpass`)

## Accessing Asterisk CLI

You can access the Asterisk CLI to monitor SIP activity, check registrations, and debug issues:

```bash
docker exec -it rustalk-asterisk asterisk -r
```

### Useful CLI Commands

Once in the Asterisk CLI:

```bash
# Show SIP registrations
pjsip show endpoints

# Show active calls
core show channels

# Show SIP peers
pjsip show contacts

# Reload configuration
pjsip reload

# Show version
core show version

# Exit CLI
exit
```

## Viewing Logs

### Asterisk Logs

Asterisk logs are stored in `asterisk-logs/` directory (excluded from git):

```bash
# View main log
tail -f asterisk-logs/asterisk.log

# View full log
cat asterisk-logs/asterisk.log

# View messages log
tail -f asterisk-logs/messages
```

### Docker Container Logs

You can also view logs directly from Docker:

```bash
# View container logs
docker logs rustalk-asterisk

# Follow logs in real-time
docker logs -f rustalk-asterisk
```

## Configuring Test Users

To add additional test users, edit `asterisk-config/pjsip.conf`:

```ini
[newuser]
type=endpoint
context=default
disallow=all
allow=ulaw
allow=alaw
transport=transport-udp
auth=newuser-auth
aors=newuser

[newuser-auth]
type=auth
auth_type=userpass
password=newpass
username=newuser

[newuser]
type=aor
max_contacts=1
contact=sip:newuser@127.0.0.1
```

After modifying configuration:

1. Reload Asterisk configuration:
   ```bash
   docker exec -it rustalk-asterisk asterisk -rx "pjsip reload"
   ```

2. Or restart the container:
   ```bash
   npm run docker:stop
   npm run docker:start
   ```

## Running Integration Tests

Integration tests that require the SIP server will automatically use the local Asterisk instance if it's running.

### With Docker Running

```bash
# Start Asterisk
npm run docker:start

# Run Rust integration tests
cd src-tauri && cargo test --test sip_transport_integration_test

# Run all tests
cd src-tauri && cargo test
```

### Without Docker (Skip SIP Tests)

If you want to run tests without Docker (e.g., in CI without Docker support):

```bash
# Set environment variable
export SKIP_SIP_INTEGRATION_TESTS=true

# Run tests (SIP tests will be skipped)
cd src-tauri && cargo test
```

## Troubleshooting

### Container Won't Start

**Problem**: Docker container fails to start.

**Solutions**:
1. Check Docker is running: `docker ps`
2. Check port conflicts: `lsof -i :5060` (macOS/Linux)
3. View container logs: `docker logs rustalk-asterisk`
4. Check Docker resources: Ensure Docker has enough memory (4GB+ recommended)

### Port Already in Use

**Problem**: Port 5060 or 5061 is already in use.

**Solutions**:
1. Find process using port:
   ```bash
   # macOS/Linux
   lsof -i :5060
   ```
2. Stop the conflicting service
3. Or modify `docker-compose.yml` to use different ports

### SIP Registration Fails

**Problem**: Test user cannot register with Asterisk.

**Solutions**:
1. Verify container is running: `docker ps | grep asterisk`
2. Check Asterisk logs: `docker logs rustalk-asterisk`
3. Verify configuration: `docker exec -it rustalk-asterisk asterisk -rx "pjsip show endpoints"`
4. Check network connectivity: `ping localhost`
5. Verify credentials match `asterisk-config/pjsip.conf`

### Configuration Changes Not Applied

**Problem**: Changes to `asterisk-config/` files don't take effect.

**Solutions**:
1. Reload configuration:
   ```bash
   docker exec -it rustalk-asterisk asterisk -rx "pjsip reload"
   ```
2. Or restart container:
   ```bash
   npm run docker:stop && npm run docker:start
   ```

### RTP Media Issues

**Problem**: Audio doesn't work in calls.

**Solutions**:
1. Verify RTP port range is open: `10000-10029/udp`
2. Check firewall settings
3. Verify RTP configuration in `asterisk-config/rtp.conf`
4. Check Asterisk logs for RTP errors

## Architecture

### Directory Structure

```
rustalk/
├── docker-compose.yml          # Docker Compose configuration
├── asterisk-config/            # Asterisk configuration files
│   ├── pjsip.conf             # PJSIP endpoint configuration
│   ├── extensions.conf        # Dialplan
│   ├── modules.conf           # Module loading
│   └── rtp.conf               # RTP configuration
└── asterisk-logs/             # Asterisk log files (git-ignored)
```

### Configuration Files

- **pjsip.conf**: Defines SIP endpoints, authentication, and transports
- **extensions.conf**: Defines dialplan for call routing
- **modules.conf**: Controls which Asterisk modules are loaded
- **rtp.conf**: Configures RTP port range for media streams

### Docker Network

The Asterisk container runs on a bridge network (`rustalk-network`) for isolation. The container is accessible from the host machine via `localhost`.

## Best Practices

1. **Always start Docker before running tests**: Use `npm run docker:start` or let `npm run tauri:dev` handle it
2. **Check logs when debugging**: Use `docker logs rustalk-asterisk` or Asterisk CLI
3. **Keep test credentials separate**: Never commit real credentials to git
4. **Clean up when done**: Use `npm run docker:stop` to stop containers when not needed
5. **Version control config, not logs**: `asterisk-config/` is tracked, `asterisk-logs/` is ignored

## Next Steps

- See [setup.md](./setup.md) for general development environment setup
- See [05-implementation-roadmap.md](../architecture/05-implementation-roadmap.md) for development roadmap
- Check integration tests in `src-tauri/tests/` for examples of using the SIP server

---

**Last Updated**: 2025-11-18  
**Asterisk Version**: Latest stable (via `andrius/asterisk:latest`)  
**Docker Compose Version**: 3.8

