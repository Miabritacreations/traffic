# Decentralized Traffic Rerouting App

## Overview

This project implements a **decentralized traffic rerouting app** built on the Internet Computer Protocol (ICP). The app is designed to crowdsource real-time traffic data, allowing users to avoid congestion, report road conditions, and earn rewards for their contributions.

With gamified elements and blockchain-backed accuracy, the app transforms daily driving into a dynamic, rewarding, and community-driven experience.

---

## Features

### Core Functionalities
1. **Traffic Reports**:
   - Users can create, read, update, and delete traffic reports.
   - Reports include details like description, location, severity, and timestamp.
   - Seamlessly update road statuses or resolve issues once addressed.

2. **User Profiles**:
   - Profiles track user contributions, points, and rewards (Route Tokens).
   - Users earn points for submitting reports and contributing to the community.
   - CRUD operations allow managing user accounts dynamically.

3. **Gamified Experience**:
   - Users are rewarded for contributing to the app, earning points and tokens.
   - Contributions like accurate reporting or verifying conditions are incentivized.

4. **Decentralized and Secure**:
   - Built on the ICP for decentralized storage and logic execution.
   - Data integrity is maintained through blockchain technology.

### Error Handling
- Robust error messages for CRUD operations:
  - `NotFound`: Data (report or user) not found in the system.
  - `AlreadyExists`: Duplicate entries are not allowed.
  - `OperationFailed`: General errors during updates or data modifications.

---

## Technologies Used

- **Programming Language**: Rust
- **Framework**: Internet Computer SDK (IC SDK)
- **Libraries**:
  - `ic-stable-structures`: For stable memory storage.
  - `serde` and `candid`: For serialization and deserialization.
  - `ic_cdk`: For ICP-specific operations.

---

## Prerequisites

- [DFINITY SDK](https://sdk.dfinity.org/)
- [Rust](https://www.rust-lang.org/) (latest stable version)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)

## Project Setup

### 1. Install DFINITY SDK

```bash
sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
```

### 2. Install Rust and add the WebAssembly target

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
```

### 3. Clone and Navigate to the Project

```bash
git clone https://github.com/miabritacreations/traffic.git
cd traffic
```

### 4. Project Structure

Ensure your project has the following structure:

```
dairy/
├── Cargo.toml
├── dfx.json
└── src/
    └── dairy_backend/
           ├── src
           |    └── lib.rs
           └── dairy_backend.did
```

## Building and Deploying

1. Start the local Internet Computer network:

```bash
dfx start --background
```

2. Deploy the canister:

```bash
dfx deploy
```

## Accessing the Candid UI

You can access the Candid UI to interact with your canister visually. After deploying, the console will display a link similar to:

```
http://127.0.0.1:8000/?canisterId=<canister-id>
```

Replace `<canister-id>` with the actual canister ID from the deployment output.