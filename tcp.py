import socket
print("connecting ...")
host = "127.0.0.1"
port = 100                 
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect((host, port))
CREATE_WALLET = b'CreateWallet\n{"address":"jane123","password":"12345","wallet_name":"sudo"}\n0\n0\n0\n'
TRANSFER = b'Transfer\n{"sender":"brunominto","receiver":"jane123","amount":"1.2","transaction_id":"118990f999","sender_password":"123456"}\n0\n0\n0\n'
GET_BALANCE = b'GetBalance\n{"address":"brunominto"}\n0\n0\n0\n'
s.sendall(GET_BALANCE)
data = s.recv(1024)
print('Received', repr(data))
s.close()
