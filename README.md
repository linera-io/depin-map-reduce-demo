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
linera net up --with-faucet --faucet-port 8081 &
LINERA_FAUCET_URL=http://localhost:8081
```

Alternatively, to use the Linera testnet, one may write:
```
LINERA_FAUCET_URL=https://faucet.testnet-babbage.linera.net
```

A wallet needs to be initialized to handle the microchains, so a temporary one will be created:

```
WALLET_DIR="$(mktemp -d)"
export LINERA_WALLET="${WALLET_DIR}/wallet.json"
export LINERA_KEYSTORE="${WALLET_DIR}/keystore.json"
export LINERA_STORAGE="rocksdb:${WALLET_DIR}/storage.db"
linera wallet init --faucet "$LINERA_FAUCET_URL" --with-new-chain
```

The wallet already contains a default microchain. This chain will be used as the root chain.

```
ROOT_CHAIN="$(linera wallet show --short)"
```

An edge chain must also be created.

```
EDGE_CHAIN="$(linera open-chain --initial-balance 50 | sed '2q;d')"
```

The application must then be deployed on the root chain.

```
APP_ID="$(linera project publish-and-create .)"
```

To interact with the application, a node service must be kept running

```
linera service --port 8080 &
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

VALUE="$(\
    curl "http://127.0.0.1:8080/chains/${ROOT_CHAIN}/applications/${APP_ID}" \
        --data '{"query": "query { value }"}' \
)"
test "$VALUE" = '{"data":{"value":15}}'
```

