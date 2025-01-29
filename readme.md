# UDP hole punching in rust

This project demonstrates UDP hole punching in rust.

> [!WARNING]  
> Do not use this code in production.

## What is hole punching?

Usually, clients that are behind a firewall / router / NAT cannot be connected directly to.
This is because connections from the outside will only be forwarded to the client if once
the client sent a package to that connection first. This makes is hard to implement peer to
peer connections. Hole punching is one way to solve this issue an therefore allows two peers
to talk to each other, even if they are usually not reachable from the outside.

## How does it work?

Hole punching works by employing a third party that can be connected to from both clients,
usually just a normal server. Both clients talk to the server, which then exchanges the
address information. Then both clients send each other packages directly, which will fail
at first, but after each client has sent at least one package, following ones are let
through.

## How to run?

You will need three devices. One of them needs to be reachable from the outside.

Start the server with:

```bash
cargo run --bin server
```

Start client a with:

```bash
cargo run --bin client <ip or domain of your sever> <name client a>
```

Start client b with:

```bash
cargo run --bin client <ip or domain of your sever> <name client b> <name client a>
```


## Detailed process

This section describes the protocol used in this demonstration. Other implementations
might be slightly different, but the concept is the same.

The process can be split in two, a `registering` phase and a `hole punching` phase.

### Registering

This is done because the server needs to know which clients are available.  
Both clients need to register.

1. The `client` sends a `Register` package, telling the server it's name.  
   The server remembers that name and corresponding address (IP and port)
2. The `server` sends a `RegisterAck` package.
   This is just done for convenience.
   
### Hole punching

1. `client_a` sends a `HolePunchRequest` to the `server`
2. The `server` checks if the both clients are known.
   - If not, he responds with `HolePunchResponseUnknown`
     and the process failed.
3. The `server` sends both clients a `HolePunchInitiate` packet containing
   the address of the other client.
4. Both `clients` start continuously sending `Message` packets to the
   address received by the server.
   