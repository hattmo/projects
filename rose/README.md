# Rooftop Sentry

## Overview

Hypervisor escape detection tool

## Installation

Add the gitea cargo registry to `~/.cargo/config.toml`
```toml
[registries.gitea]
index = "sparse+http://192.168.80.231:3000/api/packages/KVM/cargo/"
```
Add your credentials to `~/.cargo/credentials.toml`
```toml
[registries.gitea]
token = "Bearer ${APITOKEN}"
```
Run the following in your shell
```bash
cargo install --registry gitea --target x86_64-unknown-linux-musl rose
```
