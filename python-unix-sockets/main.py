from socket import socket, SOCK_STREAM, AF_UNIX
socket_path = '/home/davidmaceachern/github/nodesozu/tmp/sock'
clientSocket = socket(AF_UNIX, SOCK_STREAM)

def recv():
    print(clientSocket.recv(1024).decode())

clientSocket.connect(socket_path)
recv()

print('connected')

clientSocket.send('test\r\n'.encode())
recv()

print('Sent')
