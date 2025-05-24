from decimal import Decimal
import socket  # not used, can remove
import json
import hashlib
import time
import requests
from ecdsa import SigningKey, SECP256k1
from ecdsa.util import sigencode_string_canonize

# ------------------ User Class ------------------

class User:
    def __init__(self, wallet_name: str, address: str, seed_phrase: str):
        self.wallet_name = wallet_name
        self.address = address
        self.seed_phrase = seed_phrase
        self.public_key = ""

    def generate_keys_from_string(self):
        seed = hashlib.sha256(self.seed_phrase.encode()).digest()
        private_key = SigningKey.from_string(seed, curve=SECP256k1)
        public_key = private_key.verifying_key
        self.public_key = public_key.to_string("compressed").hex()
        return private_key, public_key

# ------------------ Users ------------------

genesis_user = User("genesis", "genesis", "hello hello")
user1 = User("mandy", "towerbb", "hello2")
user2 = User("greexy", "cuppythato", "moneybig holluccy")

# ------------------ HTTP Config ------------------

LOCAL_API_URL = "http://127.0.0.1:3000"  # change to your server URL and port
TEST_API_URL = "http://155.138.221.87:3000"
# ------------------ Helpers ------------------

def sign_transaction(sender: str, receiver: str, amount: str, timestamp: int, tx_id: str, private_key):
    tx_data = f"{sender}{receiver}{amount}{timestamp}{tx_id}"
    signature = private_key.sign_deterministic(
        tx_data.encode('utf-8'),
        hashfunc=hashlib.sha256,
        sigencode=sigencode_string_canonize
    )
    return signature.hex()

def get_tx_id(sender: str, receiver: str, amount: str, ts: int) -> str:
    h = hashlib.sha256()
    h.update(ts.to_bytes(8, byteorder="big", signed=True))
    h.update(sender.encode("utf-8"))
    h.update(receiver.encode("utf-8"))
    h.update(amount.encode("utf-8"))
    return h.hexdigest()

def sign_data(data: str, private_key) -> str:
    signature = private_key.sign_deterministic(
        data.encode(),
        hashfunc=hashlib.sha256,
        sigencode=sigencode_string_canonize
    )
    return signature.hex()

# ------------------ Payload Builders ------------------

def build_wallet_payload(user: User) -> dict:
    user.generate_keys_from_string()
    return {
            "address": user.address,
            "wallet_name": user.wallet_name,
            "public_key": user.public_key
        }

def build_get_user_transactions_payload(user: User) -> dict:
    return {
        "action": "get_user_transactions",
        "data": {
            "address": user.address
        }
    }

def build_transaction_payload(sender: User, receiver: str, amount: Decimal) -> dict:
    priv, _ = sender.generate_keys_from_string()
    amount_str = format(amount.normalize(), 'f')
    ts_seconds = int(time.time())
    tx_id = get_tx_id(sender.address, receiver, amount_str, ts_seconds)
    signature = sign_transaction(sender.address, receiver, amount_str, ts_seconds, tx_id, priv)
    return {
            "sender": sender.address,
            "receiver": receiver,
            "amount": amount_str,
            "timestamp": ts_seconds,
            "id": tx_id,
            "signature": signature
    }

def build_verify_wallet_payload(user: User) -> dict:
    priv, _ = user.generate_keys_from_string()
    message = "hello blockchain"
    signature = sign_data(message, priv)
    return{
            "address": user.address,
            "message": message,
            "signature": signature
        }

def build_get_last_block_height() -> dict:
    return {"action": "get_last_block_height", "data": {}}

def build_get_last_block() -> dict:
    return {"action": "get_last_block", "data": {}}

def build_get_all_blocks() -> dict:
    return {"action": "get_all_blocks", "data": {}}

def build_get_account_payload(user: User) -> dict:
    return {"address": user.address}

def get_balance_payload(user: User) -> dict:
    return {"address": user.address}

def build_mempool_payload() -> dict:
    return {"action": "get_mempool", "data": {}}

def build_hello_payload() -> dict:
    return {"action": "hello", "data": {}}

# ------------------ HTTP Send ------------------

def send_payload_http(route:str,payload: dict):
    url = f"{TEST_API_URL}/{route}"
    print("\n📦 Sending Payload:", json.dumps(payload))
    try:
        response = requests.post(url, json=payload, timeout=10)
        response.raise_for_status()
        print("📥 Response:", json.dumps(response.json(), indent=2))
    except requests.RequestException as e:
        print("❌ HTTP Request failed:", e)
        if e.response is not None:
            print("Status Code:", e.response.status_code)
            print("Body:", e.response.text)

# ------------------ Main Logic ------------------

if __name__ == "__main__":
    # Test calls:
    #send_payload_http("wallet/create_wallet",build_wallet_payload(genesis_user))
    #send_payload_http("wallet/transfer", build_transaction_payload(genesis_user, "towerbb", Decimal("100.00")))
    send_payload_http("wallet/get_account",build_get_account_payload(user1))
    #send_payload_http("wallet/get_balance", get_balance_payload(user1))
    #send_payload_http("wallet/verify_account", build_verify_wallet_payload(user1))

    TEST_API_URL