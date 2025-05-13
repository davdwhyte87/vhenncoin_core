from decimal import Decimal
import socket
import json
import hashlib
from bip_utils import Bip39MnemonicGenerator, Bip39SeedGenerator, Bip44, Bip44Coins, Bip44Changes
from ecdsa import SigningKey, SECP256k1
import os
from ecdsa.util import sigencode_string

class User:
    def __init__(self, wallet_name: str, address:str, seed_phrase:str):
        self.wallet_name = wallet_name
        self.address = address
        self.seed_phrase = seed_phrase
        self.public_key =""

    def generate_keys_from_string(self):
        # Hash the string (sha256 for example)
        hashed = hashlib.sha256(self.seed_phrase.encode()).digest()
        
        # Generate Private Key
        private_key = SigningKey.from_string(hashed, curve=SECP256k1)
        
        # Derive Public Key
        public_key = private_key.verifying_key

        self.public_key = public_key.to_string("compressed").hex()

        print("generating keys ................................")
        return private_key, public_key

genesis_user = User("genesis man", "genesis", "rolly golly folly tonny")
user1 = User("mandy scuu", "towerbb", "tennytony78389")
user2 = User("greexy", "cuppythato", "moneybig holluccy")


ip_address = "192.168.66.222"
port = 3000



# Create Transaction Payload
def create_transaction(sender, receiver, amount, nonce):
    return {
        "action":"transfer",
        "data":{
            "sender": sender,
            "receiver": receiver,
            "amount": str(amount),
            "nonce": nonce,
            "signature":""
        }
      
    }

def create_wallet_req(user:User):
    user.generate_keys_from_string()
    return {
        "action":"create_wallet",
        "data":{
            "address":user.address,
            "wallet_name":" well oman",
            "public_key":user.public_key
        }
    }

def create_get_account_req(user:User):
    return {
        "action":"get_account",
        "data":{
            "address":user.address
        }
    }




def sign_transaction2(sender: str, receiver: str, amount: str, nonce: str, private_key: SigningKey):
    # Concatenate same as Rust
    tx_data = f"{sender}{receiver}{amount}{nonce}"
    gg = "hello"
    print("tx dig data "+tx_data)
    # Hash with sha256
    tx_hash = hashlib.sha256(tx_data.encode()).digest()
    print("tx_hash:", tx_hash.hex())
    # Sign the hash
    signature = private_key.sign_deterministic(
    tx_hash,
    hashfunc=hashlib.sha256,
    sigencode=sigencode_string
    )

    # Return hex encoded signature
    return signature.hex()


# Send Transaction over TCP
def send_transaction(sender:User, receiver:str, amount:str,nonce:str):
    #amount = Decimal("500.0")
    x = amount
    #req = create_transaction(sender.address, receiver, amount , nonce)

    req = {
        "action":"transfer",
        "data":{
            "sender": sender.address,
            "receiver": receiver,
            "amount": x,
            "nonce": nonce,
            "signature":""
        }
      
    }
    priv, pub = sender.generate_keys_from_string()

    signature = sign_transaction2(sender.address, receiver, x, nonce, priv)
    req["data"]["signature"] = signature
    data = json.dumps(req).encode()
    return data


def get_mempool():
    req ={
        "action":"get_mempool",
        "data":{}
    }

    data = json.dumps(req).encode()
    return data
    


def send_create_wallet(user:User):
    # generate keys 
    
    req =create_wallet_req(user)
    data = json.dumps(req).encode()
    return data

def send_get_account():
    req = create_get_account_req(user1)
    data = json.dumps(req).encode()
    return data

# ------------------------------

if __name__ == "__main__":
   #data = send_create_wallet()

   amt = str(Decimal("900.0"))
   data = send_transaction(user1, user2.address, amt , "0")

   #data = send_get_account()

   #data = get_mempool()

   print(data)
   with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((ip_address, port))  # Change to your server IP/Port
        s.sendall(data)
        s.shutdown(socket.SHUT_WR) 
        print("data sent!")
         # Read response
        response = b""
        while True:
            chunk = s.recv(4096)
            if not chunk:
                break
            response += chunk
        
        response_data = json.loads(response.decode())
        print("Response:", response_data)