# 🚀 VhennCoin

**VhennCoin** is a lightweight, Rust-based blockchain designed for fast, secure, and scalable transactions. It supports wallet creation, transaction signing, account querying, and more — built with decentralization and education in mind.

---

## 📦 Features

- 🧾 Transaction signing and verification (ECDSA - SECP256k1)
- 🔗 Simple block structure with hash chaining
- ⛓️ Mempool management
- 🧑‍💻 Wallets with flexible seed phrase (eg.. marryana# welinton park boys)
- 🔐 Public/private key generation 
- Human readable wallet address (eg. kennyblock_345#, denny_jones_groups_mmg3, bally900_)
- 📄 JSON-based TCP message protocol

---

## ⚙️ Running the Server

> Requires **Rust** (2021 edition or newer)

### 1. Clone the repo

```bash
git clone https://github.com/your-username/vhenncoin.git
cd vhenncoin
```
### 2. Run the server
```bash
cargo build --release

cargo run --release
```

### Run the client 
```
python3 client.py
```

## API actions 
Get all blocks
``` json
{
"action":"get_all_blocks",
"data":{}
}
```

Get mempool
``` json
 {
    "action": "get_mempool",
    "data": {}
 }
```

Get user account 
``` json
{
    "action": "get_account",
    "data": {
        "address": user.address
    }
  }
```

Get last block
``` json
{
    "action":"get_last_block",
    "data":{}
}
```

Get last block height
``` json
{
    "action":"get_last_block_height",
    "data":{}
}
```

Transfer 
``` json
{
    "action": "transfer",
    "data": {
        "sender": sender.address,
        "receiver": receiver,
        "amount": amount_str,
        "nonce": nonce,
        "signature": signature
    }
}
```
Get user transactions
``` json
 {
        "action": "get_user_transactions",
        "data": {
            "address": user.address
        }
    }
```

Create wallet
``` json
{
    "action": "create_wallet",
    "data": {
        "address": user.address,
        "wallet_name": user.wallet_name,
        "public_key": user.public_key
    }
}
```

## 🤝 Contributing
Pull requests are welcome! Feel free to fork the repo, improve the blockchain logic, add new features, or create a web dashboard.

## 🔐 License
MIT License © 2025 [Your Name or Team Name]