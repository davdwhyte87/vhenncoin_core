import shutil
import socket
print("connecting ...")
host = "127.0.0.1"
port = 100                 
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect((host, port))
CREATE_WALLET = b'CreateWallet\n{"address":"david123","password":"12345","wallet_name":"david"}\n0\n0\n0\n'
TRANSFER = b'Transfer\n{"sender":"hkdoor","receiver":"david123","amount":"50000000000.0","transaction_id":"3738h84hf874","sender_password":"12345"}\n0\n0\n0\n'
GET_BALANCE = b'GetBalance\n{"address":"david123"}\n0\n0\n0\n'
GET_WALLET = b'GetWalletData\n{"address":"david123"}\n0\n0\n0\n'
GET_CHAIN_ZIP = b'GetZipChain\n0\n0\n0\n0\n'

CREATE_USER_ID = b'CreateUserId\n{"user_name":"nubule#$erro","password":"12345"}\n0\n0\n0\n'
VALIDATE_USER_ID = b'ValidateUserId\n{"user_name":"david123","password":"12345"}\n0\n0\n0\n'

s.sendall(CREATE_USER_ID)
data = s.recv(4000)

# with open('download.zip','wb') as out:
#     inp = s.makefile('rb')
#     shutil.copyfileobj(inp, out)

print('Received', repr(data))

s.close()
