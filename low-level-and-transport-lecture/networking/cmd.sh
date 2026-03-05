# UDP server
socat UDP-RECV:48772 STDOUT

# UDP client
socat STDIO UDP:127.0.0.1:48772

# TCP server
nc -l -p 8888 -t

# TCP client
nc localhost 8888
