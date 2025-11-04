# Rustalk Development Container
# Based on Debian 12 (bookworm) with Rust, Node.js 20, and Tauri dependencies

FROM debian:bookworm-slim

# Install basic dependencies and tools
RUN apt-get update && apt-get install -y \
	curl \
	git \
	build-essential \
	pkg-config \
	libssl-dev \
	ca-certificates \
	zsh \
	&& rm -rf /var/lib/apt/lists/*

# Install Node.js 20 LTS from NodeSource
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
	apt-get install -y nodejs && \
	rm -rf /var/lib/apt/lists/*

# Install Rust using rustup (non-interactive)
ENV RUSTUP_HOME=/usr/local/rustup \
	CARGO_HOME=/usr/local/cargo \
	PATH=/usr/local/cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable && \
	chmod -R a+w $RUSTUP_HOME $CARGO_HOME

# Install Tauri system dependencies
# Note: These are Linux dependencies. For macOS/Windows builds, Tauri uses host system libraries
# The container is primarily for development and testing
RUN apt-get update && apt-get install -y \
	libwebkit2gtk-4.1-dev \
	libgtk-3-dev \
	librsvg2-dev \
	libayatana-appindicator3-dev \
	&& rm -rf /var/lib/apt/lists/*

# Install audio dependencies for cpal (Linux support)
# macOS/Windows builds will use host system libraries
RUN apt-get update && apt-get install -y \
	libasound2-dev \
	portaudio19-dev \
	&& rm -rf /var/lib/apt/lists/*

# Install Rust development tools
RUN cargo install cargo-nextest cargo-llvm-cov cargo-audit --locked

# Install Tauri CLI globally
RUN npm install -g @tauri-apps/cli@latest

# Create non-root user for development
RUN groupadd -r vscode && useradd -r -g vscode -s /bin/zsh vscode && \
	mkdir -p /home/vscode /workspace && \
	chown -R vscode:vscode /home/vscode /workspace

# Set working directory
WORKDIR /workspace

# Switch to non-root user
USER vscode

# Verify installations
RUN rustc --version && \
	cargo --version && \
	node --version && \
	npm --version && \
	cargo tauri --version || echo "Tauri CLI will be available after project initialization"

# Default shell
SHELL ["/bin/zsh", "-c"]



