#!/usr/bin/env python

from socket import socket, AF_INET, SOCK_DGRAM
import random

sock = socket(AF_INET, SOCK_DGRAM)

def buildmsg():
    msg_type = random.choice(["ms", "c", "g"])
    msg_val = random.randint(0, 1000)
    msg_name = random.choice(["foo", "bar", "baz", "whizzle"])
    return msg_name + ":" + str(msg_val) + "|" + msg_type

while True:
    msg = buildmsg()
    print sock.sendto(msg, ("127.0.0.1", 8125))
