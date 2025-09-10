# 🚀 Hackathon Prerequisites Setup Guide

This guide will help you install all the necessary tools to **run a Cosmos SDK chain** and **develop & deploy CosmWasm smart contracts**.  

Please follow the steps carefully in the order given.  

---

## 1. Install Go
- Download the appropriate installer: [https://go.dev/dl/](https://go.dev/dl/)
- Follow the on-screen instructions to complete installation.

---

## 2. Install Git
- Download Git for Windows: [https://git-scm.com/downloads/win](https://git-scm.com/downloads/win)
- Install with default settings.

---

## 3. Install Visual Studio Code (VS Code)
- Download from: [https://code.visualstudio.com/download](https://code.visualstudio.com/download)
- After installation, install the **Cosmy Wasmy** extension:  
  👉 [Cosmy Wasmy Extension](https://marketplace.visualstudio.com/items?itemName=spoorthi.cosmy-wasmy)

📂 Extension GitHub: [https://github.com/spoo-bar/cosmy-wasmy](https://github.com/spoo-bar/cosmy-wasmy)

---

## 4. Install Rust
- Download and run the installer: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)  
  (Download the `rustup-init.exe` file)
- After installation, run the following commands in **Command Prompt (cmd)**:

```bash
rustup install 1.86
rustup default 1.86
rustup target add wasm32-unknown-unknown
cargo install wasm-opt
cargo install cosmwasm-check
```

---

## 5. Install Docker & WSL

* Install Docker Desktop: [https://docs.docker.com/desktop/setup/install/windows-install/](https://docs.docker.com/desktop/setup/install/windows-install/)
* Enable WSL by running in **cmd**:

```bash
wsl --install
```

⚠️ If you face WSL installation issues, run:

```bash
bcdedit /set hypervisorlaunchtype auto
```
and restart your machine, it should start working.

Reference: [WSL Issue Fix](https://github.com/microsoft/WSL/issues/9652#issuecomment-1474858120)

---

## 6. Setup the Cosmos Chain with Docker

* Pull the prebuilt blockchain image:

```bash
docker pull soumithbasina/wfblockchain:latest
```

* Initialize the chain:

```bash
docker run --rm -it --mount type=volume,source=wasmd_data,target=//root/.wasmd \
  soumithbasina/wfblockchain:latest //opt/setup_wasmd.sh
```

* Run the chain:

```bash
docker run --rm -it -p 26657:26657 -p 26656:26656 -p 1317:1317 \
  --mount type=volume,source=wasmd_data,target=//root/.wasmd \
  soumithbasina/wfblockchain:latest //opt/run_wasmd.sh
```

* To reset the blockchain state:

```bash
docker volume rm -f wasmd_data
```

---

## 7. Install Node.js & npm

* Download the prebuilt Node.js + npm binaries: [https://nodejs.org/en/download/](https://nodejs.org/en/download/)
* Complete installation with default options.

---

## payloads

```json
instantiate
{
    "oracle_pubkey": "A/NErFglPqNVghcdRNo2f983YfPIziZE0xVWBb5phEXj",
    "oracle_key_type": "secp256k1"
}

queries
{"get_oracle_data": {}}
{"get_oracle_pubkey": {}}
{"get_admin": {}}

execute 
{
  "send": {
    "recipient": "wasm175vkrltwkfshvqa539xzu7gwqjppz8p5ltuj4x"
  }
}

```

# ✅ You are now ready!

With the above setup, you can:

* Run a local Cosmos SDK blockchain.


