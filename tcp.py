import shutil
import socket
print("connecting ...")
host = "10.255.255.254"
live_host = "155.138.224.183"
port = 3000                 
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect((live_host, port))
CREATE_WALLET = b'CreateWallet\n{"address":"mimikkok","password":"","wallet_name":"", "vcid_username":"pamela808#", "is_vcid":false}\n0\n0\n0\n'
TRANSFER = b'Transfer\n{"sender":"mimikkok","receiver":"danny_f","amount":"400000.0","transaction_id":"uiw983infH__jsknfskjs","sender_password":"123456"}\n0\n0\n0\n'
GET_BALANCE = b'GetBalance\n{"address":"danny_f"}\n0\n0\n0\n'
GET_WALLET = b'GetWalletData\n{"address":"mimikkok"}\n0\n0\n0\n'
GET_CHAIN_ZIP = b'GetZipChain\n0\n0\n0\n0\n'

CREATE_USER_ID = b'CreateUserId\n{"user_name":"nino88","password":"12345"}\n0\n0\n0\n'
VALIDATE_USER_ID = b'ValidateUserId\n{"user_name":"pamela808#","password":"123456"}\n0\n0\n0\n'

s.sendall(CREATE_USER_ID)
data = s.recv(4000)

# with open('download.zip','wb') as out:
#     inp = s.makefile('rb')
#     shutil.copyfileobj(inp, out)

print('Received', repr(data))

s.close()
