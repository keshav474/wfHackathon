# 🚀 Hackathon Setup Guide

This guide will help you install all the necessary tools to **run a Cosmos SDK chain** and **develop & deploy CosmWasm smart contracts**.  

Please follow the steps carefully in the order given. 

# Prerequisites 
## 1. Install Go
- Download the appropriate installer: [https://go.dev/dl/](https://go.dev/dl/)
- Follow the on-screen instructions to complete installation.

---

## 2. Install Git
- Download Git for Windows: [https://git-scm.com/downloads/win](https://git-scm.com/downloads/win)
- Install with default settings.
- Unless mentioned otherwise, use **Git Bash** for running the commands

---

## 3. Install Visual Studio Code (VS Code)
- Download from: [https://code.visualstudio.com/download](https://code.visualstudio.com/download)
- After installation, install the **Cosmy Wasmy** extension:  
  👉 [Cosmy Wasmy Extension](https://marketplace.visualstudio.com/items?itemName=spoorthi.cosmy-wasmy)

📂 Extension GitHub: [https://github.com/spoo-bar/cosmy-wasmy](https://github.com/spoo-bar/cosmy-wasmy)
- Add this to `cosmywasmy.chains` within `settings.json` of vscode
```json
{
    "configName": "wfchain",
    "chainId": "testing",
    "chainEnvironment": "localnet",
    "addressPrefix": "wasm",
    "rpcEndpoint": "http://localhost:26657",
    "defaultGasPrice": "0",
    "chainDenom": "ustake"
},
```
- To open `settings.json`, open VS Code settings, search for cosmywasmy and click on `Edit in settings.json` 
- Click on the box in the bottom-left of the editor and click on wfchain in the prompt that pops up.
---

## 4. Install Rust
- Download and run the installer: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)  
  (Download the `rustup-init.exe` file)
- After installation, run the following commands in Terminal:

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

⚠️ If you face WSL installation issues, run in **cmd** with admin privileges:

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
* Install `cosmwasm-ts-codegen`: 
```bash
npm install -g @cosmwasm/ts-codegen@1.6.0
```

---

# Compiling and deploying the Smart Contract
## Compile the smart contract
Change working directory
```bash
cd sample-contract
```
Build WASM
```bash
RUSTFLAGS="-C link-arg=-s" cargo wasm
```
Confirm `sample_contract.wasm` is listed
```bash
ls target/wasm32-unknown-unknown/release/sample_contract.wasm
```
Optimize WASM
```bash
wasm-opt -Os --signext-lowering ./target/wasm32-unknown-unknown/release/sample_contract.wasm -o ./target/wasm32-unknown-unknown/release/sample_contract_opt.wasm
```
Validate sample_contract_opt.wasm
```bash
cosmwasm-check ./target/wasm32-unknown-unknown/release/sample_contract_opt.wasm
```

## Deploy the smart contract
By now the optimized contract should be ready. The chain also must be running in the background.

## payloads

```json
instantiate
{
    "oracle_pubkey": "AjrX9BclyF9K8drtbJ+0+FBbGsS4Pg+UjPiYfBT7nRh2",
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

{
    "update_oracle": {
        "new_pubkey": "AjrX9BclyF9K8drtbJ+0+FBbGsS4Pg+UjPiYfBT7nRh2",
        "new_key_type": "secp256k1"
    }
}
```

```bash
RUSTFLAGS="-C link-arg=-s" cargo wasm
wasm-opt -Os --signext-lowering ./target/wasm32-unknown-unknown/release/sample_contract.wasm -o ./target/wasm32-unknown-unknown/release/sample_contract_opt.wasm
cosmwasm-check ./target/wasm32-unknown-unknown/release/sample_contract_opt.wasm 

cosmwasm-ts-codegen generate \
    --plugin client \
    --schema ./schema \
    --out ../oracle-service/src/sdk \
    --name oracle
```

# ✅ You are now ready!

With the above setup, you can:

* Run a local Cosmos SDK blockchain.


