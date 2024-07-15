import shutil
import socket
print("connecting ...")
host = "127.0.0.1"
port = 100                 
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect((host, port))
CREATE_WALLET = b'CreateWallet\n{"address":"tom123","password":"12345","wallet_name":"david"}\n0\n0\n0\n'
TRANSFER = b'Transfer\n{"sender":"david123","receiver":"tom123","amount":"1.2","transaction_id":"118990f999","sender_password":"12345"}\n0\n0\n0\n'
GET_BALANCE = b'GetBalance\n{"address":"david123"}\n0\n0\n0\n'
GET_CHAIN_ZIP = b'GetZipChain\n0\n0\n0\n0\n'
s.sendall(GET_BALANCE)
data = s.recv(4000)

# with open('download.zip','wb') as out:
#     inp = s.makefile('rb')
#     shutil.copyfileobj(inp, out)

print('Received', repr(data))

s.close()
