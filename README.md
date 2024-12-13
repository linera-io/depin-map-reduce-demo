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
