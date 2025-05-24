from decimal import Decimal
import socket
import json
import hashlib
from bip_utils import Bip39SeedGenerator
from ecdsa import SigningKey, SECP256k1
from ecdsa.util import sigencode_string, sigencode_string_canonize
import time

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
genesis_live_user = User("", "genesis", "")
user1 = User("mandy", "towerbb", "hello2")
user2 = User("greexy", "cuppythato", "moneybig holluccy")

# ------------------ Network Config ------------------

local_ip_address = "127.0.0.1"
test_ip_address = "155.138.221.87"
port = 3000
live_ip_address = "155.138.221.87"
live_port = 9990

# ------------------ Helpers ------------------

def sign_transaction(sender:str, receiver:str, amount:str, timestamp:int, id:str, private_key):
    tx_data = f"{sender}{receiver}{amount}{timestamp}{id}"
    print("🔏 tx_data to sign:", tx_data)
    print("raw hex:", tx_data.encode('utf-8').hex())
    signature = private_key.sign_deterministic(
        tx_data.encode('utf-8'),
        hashfunc=hashlib.sha256,
        sigencode=sigencode_string_canonize
    )
    return signature.hex()


def get_tx_id(sender:str, receiver:str, amount:str, ts:int):
    h = hashlib.sha256()
    h.update(ts.to_bytes(8, byteorder="big", signed=True))
    h.update(sender.encode("utf-8"))
    h.update(receiver.encode("utf-8"))
    h.update(amount.encode("utf-8"))

    return h.hexdigest()


def sign_data(data:str, private_key):
    tx_data = f"{data}"
    print("🔏 tx_data to sign:", tx_data)
    tx_hash = hashlib.sha256(tx_data.encode()).digest()
    print("🔐 tx_hash:", tx_hash.hex())
    signature = private_key.sign_deterministic(
        data.encode(),
        hashfunc=hashlib.sha256,
        sigencode=sigencode_string_canonize
    )
    return signature.hex()

# ------------------ Payload Builders ------------------

def build_wallet_payload(user: User):
    user.generate_keys_from_string()
    pload = {
        "action": "create_wallet",
        "data": {
            "address": user.address,
            "wallet_name": user.wallet_name,
            "public_key": user.public_key
        }
    }
    print(print(json.dumps(pload, indent=2)))
    return pload

def build_get_user_transactions_payload(user: User):
    user.generate_keys_from_string()
    return {
        "action": "get_user_transactions",
        "data": {
            "address": user.address
        }
    }

def build_transaction_payload(sender: User, receiver: str, amount: Decimal):
    priv, _ = sender.generate_keys_from_string()
    amount_str = format(amount.normalize(), 'f')
    ts_seconds = int(time.time())
    tx_id = get_tx_id(sender.address, receiver, amount_str, ts_seconds )
    signature = sign_transaction(sender.address, receiver, amount_str, ts_seconds, tx_id, priv)
    pload = {
        "action": "transfer",
        "data": {
            "sender": sender.address,
            "receiver": receiver,
            "amount": amount_str,
            "timestamp": ts_seconds,
            "id":tx_id,
            "signature": signature
        }
    }

    print(print(json.dumps(pload, indent=2)))
    return pload

def build_verify_wallet_payload(user: User):
    priv, _ = user.generate_keys_from_string()
    print("public key", user.public_key)
    data = "hello benny"
    signature = sign_data(data, priv)
    return {
        "action": "verify_wallet",
        "data": {
            "address": user.address,
            "message": data,
            "signature": signature
        }
    }


def build_getlast_block_height():
    return {
        "action":"get_last_block_height",
        "data":{}
    }
def build_getlast_block():
    return {
        "action":"get_last_block",
        "data":{}
    }

def build_get_all_blocks():
    return {
        "action":"get_all_blocks",
        "data":{}
    }

def build_get_account_payload(user: User):
    return {
        "action": "get_account",
        "data": {
            "address": user.address
        }
    }


def get_balance_payload(user: User):
    return {
        "action": "get_balance",
        "data": {
            "address": user.address
        }
    }


def build_mempool_payload():
    return {
        "action": "get_mempool",
        "data": {}
    }


def build_hello_payload():
    return {
        "action": "hello",
        "data": {}
    }

# ------------------ Socket Send ------------------

def send_payload(payload):
    data = json.dumps(payload).encode()
    print("\n📦 Sending Payload:", payload)

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((live_ip_address, live_port))
        s.sendall(data)
        s.shutdown(socket.SHUT_WR)

        print("Data sent! Awaiting response...")
        response = b""
        while True:
            chunk = s.recv(4096)
            if not chunk:
                break
            response += chunk

    try:
        decoded = json.loads(response.decode())
        print("📥 Response:", json.dumps(decoded, indent=2))
    except Exception as e:
        print("❌ Error decoding response:", e)
        print("Raw response:", response)

# ------------------ Main Logic ------------------

if __name__ == "__main__":
    # Uncomment only the request you want to test 👇

    #build_wallet_payload(user2)

    build_transaction_payload(genesis_user, "ranger_team", Decimal("200000.00"))

    #send_payload(build_get_account_payload(user1))
    #send_payload(get_balance_payload(user1))
    #send_payload(build_mempool_payload())

    #send_payload(build_getlast_block_height())
    #send_payload(build_getlast_block())
    #send_payload(build_get_user_transactions_payload(user1))
    #send_payload(build_get_all_blocks())
    #send_payload(build_hello_payload())

    #send_payload(build_verify_wallet_payload(genesis_user))

    # uud = User("greexy", "cuppythato", "berryhallen")
    # priv, pub = uud.generate_keys_from_string()
    # print("pub  key ",uud.public_key)
