# puct

```
cargo install --git https://github.com/LeoDog896/puct
```

port ducts. UDP and TCP.

`puct tcp 381.281.301.301` - establishes a UTP (TCP over UDP) connection

`puct tcp 692.412.825.504 25565` - establishes a TCP connection with tcp/25565 being the input

You can also specify ports, but 18479 will be the default:

`puct udp 481.351.485.245:5713`

The other party needs to use your IP (which you can get using `puct ip`)

## quick nat hole punching

Assume client A and B are behind NATs, and they know eachother's public IP addresses and ports.

If client A and client B send a packet to eachother in a short time, the NAT will allow them to communicate.