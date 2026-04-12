import socket
import struct
import json
from sys import argv

def fetch(addr, port, querystr):
    if querystr[0] != "?":
        querystr = "?"+querystr
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    query = b"\x00\x83" + struct.pack('>H', len(querystr) + 6) + b"\x00\x00\x00\x00\x00" + querystr.encode() + b"\x00"

    sock.connect((addr, port))

    sock.sendall(query)

    data = sock.recv(4096)

    return data[5:-1].decode("utf-8")

if len(argv) < 3:
    print("python test.py ip port [query]")
    exit(0)

ip = argv[1]
port = int(argv[2])
query = argv[3] if 3 < len(argv) else "?status&format=json"

meow = fetch(ip, port, query)

# if we get a json response, pretty print it
if meow[0] == "{" and meow[-1] == "}":
    print(json.dumps(json.loads(meow), indent = 4).replace("    ", "\t"))
else:
    print(meow)
