#!/bin/bash
# Rustalk DevContainer Setup Script
# Verifies all development tools are installed correctly

set -e

echo "🔍 Verifying Rustalk development environment..."

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "\n${BLUE}=== Tool Versions ===${NC}\n"

# Check Rust
echo -n "Rust: "
if command -v rustc &> /dev/null; then
	RUST_VERSION=$(rustc --version)
	echo -e "${GREEN}✓ ${RUST_VERSION}${NC}"
else
	echo -e "${YELLOW}✗ Not found${NC}"
	exit 1
fi

# Check Cargo
echo -n "Cargo: "
if command -v cargo &> /dev/null; then
	CARGO_VERSION=$(cargo --version)
	echo -e "${GREEN}✓ ${CARGO_VERSION}${NC}"
else
	echo -e "${YELLOW}✗ Not found${NC}"
	exit 1
fi

# Check Node.js
echo -n "Node.js: "
if command -v node &> /dev/null; then
	NODE_VERSION=$(node --version)
	echo -e "${GREEN}✓ ${NODE_VERSION}${NC}"
else
	echo -e "${YELLOW}✗ Not found${NC}"
	exit 1
fi

# Check npm
echo -n "npm: "
if command -v npm &> /dev/null; then
	NPM_VERSION=$(npm --version)
	echo -e "${GREEN}✓ ${NPM_VERSION}${NC}"
else
	echo -e "${YELLOW}✗ Not found${NC}"
	exit 1
fi

# Check Tauri CLI (installed via npm, available as 'tauri' command)
echo -n "Tauri CLI: "
if command -v tauri &> /dev/null; then
	TAURI_VERSION=$(tauri --version 2>/dev/null || echo "installed")
	echo -e "${GREEN}✓ ${TAURI_VERSION}${NC}"
elif command -v cargo-tauri &> /dev/null; then
	TAURI_VERSION=$(cargo tauri --version 2>/dev/null || echo "installed")
	echo -e "${GREEN}✓ ${TAURI_VERSION}${NC}"
else
	echo -e "${YELLOW}⚠ Not found (will be available after project initialization)${NC}"
fi

# Check Rust tools
echo -e "\n${BLUE}=== Rust Development Tools ===${NC}\n"

echo -n "cargo-nextest: "
if command -v cargo-nextest &> /dev/null; then
	echo -e "${GREEN}✓ Installed${NC}"
else
	echo -e "${YELLOW}⚠ Not found${NC}"
fi

echo -n "cargo-llvm-cov: "
if command -v cargo-llvm-cov &> /dev/null; then
	echo -e "${GREEN}✓ Installed${NC}"
else
	echo -e "${YELLOW}⚠ Not found${NC}"
fi

echo -n "cargo-audit: "
if command -v cargo-audit &> /dev/null; then
	echo -e "${GREEN}✓ Installed${NC}"
else
	echo -e "${YELLOW}⚠ Not found${NC}"
fi

# Check system dependencies
echo -e "\n${BLUE}=== System Dependencies ===${NC}\n"

echo -n "pkg-config: "
if command -v pkg-config &> /dev/null; then
	echo -e "${GREEN}✓ Installed${NC}"
else
	echo -e "${YELLOW}✗ Not found${NC}"
fi

echo -n "OpenSSL development: "
if pkg-config --exists openssl; then
	OPENSSL_VERSION=$(pkg-config --modversion openssl)
	echo -e "${GREEN}✓ ${OPENSSL_VERSION}${NC}"
else
	echo -e "${YELLOW}✗ Not found${NC}"
fi

# Check Tauri dependencies (Linux)
echo -e "\n${BLUE}=== Tauri System Libraries (Linux) ===${NC}\n"

echo -n "WebKitGTK: "
if pkg-config --exists webkit2gtk-4.1 2>/dev/null || pkg-config --exists webkit2gtk-4.0 2>/dev/null; then
	echo -e "${GREEN}✓ Installed${NC}"
else
	echo -e "${YELLOW}⚠ Not found (not required for macOS/Windows builds)${NC}"
fi

echo -n "GTK+3: "
if pkg-config --exists gtk+-3.0; then
	echo -e "${GREEN}✓ Installed${NC}"
else
	echo -e "${YELLOW}⚠ Not found (not required for macOS/Windows builds)${NC}"
fi

# Check audio dependencies
echo -e "\n${BLUE}=== Audio Libraries (cpal - Linux) ===${NC}\n"

echo -n "ALSA: "
if pkg-config --exists alsa; then
	echo -e "${GREEN}✓ Installed${NC}"
else
	echo -e "${YELLOW}⚠ Not found (not required for macOS/Windows builds)${NC}"
fi

# Install Oh My Zsh
echo -e "\n${BLUE}=== Installing Oh My Zsh ===${NC}\n"

if [ ! -d "$HOME/.oh-my-zsh" ]; then
	echo -n "Oh My Zsh: "
	sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended
	if [ -d "$HOME/.oh-my-zsh" ]; then
		echo -e "${GREEN}✓ Installed${NC}"
	else
		echo -e "${YELLOW}⚠ Installation failed${NC}"
	fi
else
	echo -e "${GREEN}✓ Oh My Zsh already installed${NC}"
fi

# Setup SSH directory and permissions
echo -e "\n${BLUE}=== Configuring SSH ===${NC}\n"

if [ -d "$HOME/.ssh" ]; then
	# Fix SSH directory permissions
	chmod 700 "$HOME/.ssh" 2>/dev/null || true
	find "$HOME/.ssh" -type f -exec chmod 600 {} \; 2>/dev/null || true
	find "$HOME/.ssh" -type d -exec chmod 700 {} \; 2>/dev/null || true
	
	echo -e "${GREEN}✓ SSH directory permissions fixed${NC}"
	
	# Configure SSH to use 1Password agent
	if [ -S "$HOME/.1password-agent.sock" ]; then
		# Create or update SSH config to use 1Password agent
		if [ ! -f "$HOME/.ssh/config" ]; then
			touch "$HOME/.ssh/config"
			chmod 600 "$HOME/.ssh/config"
		fi
		
		# Check if IdentityAgent is already configured
		if ! grep -q "IdentityAgent.*1password-agent" "$HOME/.ssh/config" 2>/dev/null; then
			cat >> "$HOME/.ssh/config" <<EOF

# 1Password SSH Agent
Host *
  IdentityAgent ~/.1password-agent.sock
  AddKeysToAgent yes
EOF
			chmod 600 "$HOME/.ssh/config"
			echo -e "${GREEN}✓ SSH config updated to use 1Password agent${NC}"
		else
			echo -e "${GREEN}✓ SSH config already configured for 1Password${NC}"
		fi
		
		# Test SSH agent connectivity
		echo -n "SSH Agent: "
		if ssh-add -l &>/dev/null; then
			echo -e "${GREEN}✓ Connected (keys available)${NC}"
		else
			echo -e "${YELLOW}⚠ Agent socket exists but no keys loaded${NC}"
		fi
	else
		echo -e "${YELLOW}⚠ 1Password SSH agent socket not found${NC}"
		echo -e "${YELLOW}  Make sure 1Password desktop app is running and SSH agent is enabled${NC}"
	fi
else
	echo -e "${YELLOW}⚠ SSH directory not mounted${NC}"
fi

# Summary
echo -e "\n${GREEN}✅ Development environment ready!${NC}\n"

echo -e "${BLUE}Next steps:${NC}"
echo "  1. Initialize Tauri + SvelteKit project (Phase 1)"
echo "  2. Run 'cargo tauri dev' to start development server"
echo "  3. Check docs/development/setup.md for detailed instructions"
echo ""

