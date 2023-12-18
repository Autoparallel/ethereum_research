# Use Fedora as the base image
FROM fedora:latest

# Update and install necessary packages
RUN dnf update -y && \
    dnf install -y curl gcc

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Set the environment variable for cargo
ENV PATH="/root/.cargo/bin:${PATH}"

# Install mdbook and mdbook-katex
RUN cargo install mdbook && \
    cargo install mdbook-katex

# Set the working directory
WORKDIR /book

# Command to run mdbook
CMD ["mdbook", "serve", "--hostname", "0.0.0.0", "--port", "8899"]
