import shutil
import socket
print("connecting ...")
host = "127.0.0.1"
port = 100                 
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect((host, port))
CREATE_WALLET = b'CreateWallet\n{"address":"pamela808","password":"","wallet_name":"", "vcid_username":"pamela808#", "is_vcid":true}\n0\n0\n0\n'
TRANSFER = b'Transfer\n{"sender":"pamela808","receiver":"temi9","amount":"100.0","transaction_id":"83u8jisiuduhsih8jbbyy","sender_password":"12345"}\n0\n0\n0\n'
GET_BALANCE = b'GetBalance\n{"address":"danny_f"}\n0\n0\n0\n'
GET_WALLET = b'GetWalletData\n{"address":"david123"}\n0\n0\n0\n'
GET_CHAIN_ZIP = b'GetZipChain\n0\n0\n0\n0\n'

CREATE_USER_ID = b'CreateUserId\n{"user_name":"pamela808#","password":"12345"}\n0\n0\n0\n'
VALIDATE_USER_ID = b'ValidateUserId\n{"user_name":"david123","password":"12345"}\n0\n0\n0\n'

s.sendall(TRANSFER)
data = s.recv(4000)

# with open('download.zip','wb') as out:
#     inp = s.makefile('rb')
#     shutil.copyfileobj(inp, out)

print('Received', repr(data))

s.close()
