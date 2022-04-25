import socket
import sys
import time


def handler(client_socket, msg):
    data = msg
    # to connect to anish
    client_socket.sendall(data)


# Create a UDS socket
sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)

# Connect the socket to the port where the server is listening
server_address = './uds_socket3'
print >>sys.stderr, 'connecting to %s' % server_address
try:
    sock.connect(server_address)
    print("connected")
    while True:
        handler(sock, "15anish")
        time.sleep(2)

    # handler(sock, "25anish7message")
    # handler(sock, "35anish")
except socket.error, msg:
    print >>sys.stderr, msg
    sys.exit(1)
