import shutil
import socket
print("connecting ...")
host = "172.20.10.2"
live_host = "155.138.224.183"
port = 3000                 
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect((host, port))
CREATE_WALLET = b'CreateWallet\n{"address":"genesis","password":"12345","wallet_name":"", "vcid_username":"", "is_vcid":false}\n0\n0\n0\n'
TRANSFER = b'Transfer\n{"sender":"david123","receiver":"drake_ob","amount":"40000.0","transaction_id":"uiw983infH__jsknfskjs","sender_password":"12345"}\n0\n0\n0\n'
GET_BALANCE = b'GetBalance\n{"address":"genesis"}\n0\n0\n0\n'
GET_WALLET = b'GetWalletData\n{"address":"mello_mall"}\n0\n0\n0\n'
GET_CHAIN_ZIP = b'GetZipChain\n0\n0\n0\n0\n'

CREATE_USER_ID = b'CreateUserId\n{"user_name":"nino88","password":"12345"}\n0\n0\n0\n'
VALIDATE_USER_ID = b'ValidateUserId\n{"user_name":"pamela808#","password":"123456"}\n0\n0\n0\n'

s.sendall(GET_BALANCE)
data = s.recv(4000)

# with open('download.zip','wb') as out:
#     inp = s.makefile('rb')
#     shutil.copyfileobj(inp, out)

print('Received', repr(data))

s.close()
