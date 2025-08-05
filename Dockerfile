FROM --platform=linux/amd64 ubuntu:24.04

ENV RUSTUP_HOME=/opt/rustup
ENV CARGO_HOME=/opt/cargo

# Install a basic environment needed for our build tools
RUN apt -yq update && \
    apt -yqq install --no-install-recommends curl ca-certificates \
        build-essential pkg-config libssl-dev llvm-dev liblmdb-dev clang cmake rsync libunwind-dev jq

# Install Rust and Cargo
ENV PATH=/opt/cargo/bin:${PATH}
COPY rust-toolchain.toml ./
RUN curl --fail https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path

# Install dfx
COPY dfx.json ./
RUN DFXVM_INIT_YES=1 DFX_VERSION=$(cat dfx.json | jq -r .dfx) sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
ENV export PATH=${HOME}/.local/share/dfx/bin:${PATH}

# Install ic-wasm
RUN cargo install ic-wasm --version 0.9.5 --locked

COPY . .

# Build
RUN make build

ENTRYPOINT [ "./release.sh" ]
