# Linera DePIN Map-Reduce Example

This is an example [Linera](https://linera.io) application to demonstrate one possible usage of
Linera for [decentralized physical infrastructure network][depin]. In this example, edge devices
publish values to the network and these values get "map-reduced" into a final value in a scalable
manner.

[depin]: https://en.wikipedia.org/wiki/Decentralized_physical_infrastructure_network

## Application Design

Each edge device has its own microchain, to which they submit values whenever they desire to. This
edge chain can be connected to a parent chain. Each parent chain can also have a parent chain,
until eventually the final chains are connected to a root chain.

The values submitted by an edge device are aggregated by the application on the edge chain. From
time to time, the aggregated value can be flushed to a parent chain. The parent chain repeats the
process, aggregating values from multiple edge chains and from time to time flushing the aggregated
value to its parent.

This design provides scalability, because the validators can run each microchain completely in
parallel. The only sequential steps are the flushing of values towards the roots, so the more that
can be done before a flush, the more scalable the application is.

## Example Usage

A minimal example of using the application would be to deploy it on two chains, with one being the
root chain that aggregates all values and an edge chain that aggregates the values from a single
device.

The `LINERA_FAUCET_URL` environment variable must be configured with the address of a faucet
running on a Linera network. This can be either the testnet, the devnet, or a localnet. A localnet
can be started with

```ignore
linera net up &
# Use the environment variables described in the output from the command above
LINERA_WALLET=... LINERA_STORAGE=... linera faucet --amount 100 --port 8081 &
export LINERA_FAUCET_URL=http://localhost:8081
```

For the steps below, the Linera testnet will be used:

```
export LINERA_FAUCET_URL=https://faucet.testnet-archimedes.linera.net
```

A wallet needs to be initialized to handle the microchains, so a temporary one will be created:

```
export WALLET_DIR="$(mktemp -d)"
export LINERA_WALLET="${WALLET_DIR}/wallet.json"
export LINERA_STORAGE="rocksdb:${WALLET_DIR}/storage.db"
linera wallet init --faucet "$LINERA_FAUCET_URL" --with-new-chain
```

The wallet already contains a default microchain. This chain will be used as the root chain.

```
export ROOT_CHAIN="$(linera wallet show --short)"
```

An edge chain must also be created.

```
export EDGE_CHAIN="$(linera open-chain --initial-balance 50 | tail -n 1)"
```

The application must then be deployed on both chains. The root chain already has the application,
because it was used to create the application. The edge chain must then request the root chain for
it.

```
export APP_ID="$(linera project publish-and-create .)"
# Send request from edge chain to root chain
linera request-application "$APP_ID" --requester-chain-id "$EDGE_CHAIN"
# Receive the request and send a reply from the root chain
linera process-inbox "$ROOT_CHAIN"
# Handle the reply to complete registration on the edge chain
linera process-inbox "$EDGE_CHAIN"
```

To interact with the application, a node service must be kept running

```
linera service &
```

Values can be submitted to the edge chain using GraphQL

```
curl "http://127.0.0.1:8080/chains/${EDGE_CHAIN}/applications/${APP_ID}" \
    --data '{"query": "mutation { submit(value: \"5\") }"}'
curl "http://127.0.0.1:8080/chains/${EDGE_CHAIN}/applications/${APP_ID}" \
    --data '{"query": "mutation { submit(value: \"10\") }"}'
```

Before flushing the aggregated value to the root chain, the edge chain must connect to by
configuring the root chain as its parent chain

```
curl "http://127.0.0.1:8080/chains/${EDGE_CHAIN}/applications/${APP_ID}" \
    --data "{\"query\": \"mutation { connectToParent(parent: \\\"${ROOT_CHAIN}\\\") }\"}"
```

The value can then be flushed to the root chain

```
curl "http://127.0.0.1:8080/chains/${EDGE_CHAIN}/applications/${APP_ID}" \
    --data '{"query": "mutation { flush }"}'
```

And once the message is received, it can queried from the root chain

```
# Ensure the flush message is received locally
sleep 3

export VALUE="$(\
    curl "http://127.0.0.1:8080/chains/${ROOT_CHAIN}/applications/${APP_ID}" \
        --data '{"query": "query { value }"}' \
)"
test "$VALUE" = '{"data":{"value":15}}'
```