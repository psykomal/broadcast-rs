# broadcast-rs

### Implement broadcast operation in rust using Message Passing Paradigm

Proposed Solution :

Use a MultiQueue which is a queue (has channel) and has a list of subscribers
RustRoutines are the abstration equivalent to : Thread + Mailbox (something like Elixir processes)
Currently RustRoutine stores the main queue tx internally (TODO: extend this in a better way such taht any RustRoutine can send and listen to any Queue)

Process:
1. Create a MultiQueue and start broadcast operation (thread keeps listening to its mailbox and broadcasts to all subscribers except the sender)
2. Create RustRoutines
3. Start Gossip() and Listening() for RustRoutines
4. Gossip() currently waits for rand time and sends msgs to the MultiQueue
5. Other RustRoutines listen and print the message

Disclaimer: This is an assignment for the Empowered Programmer Cohort. An attempt to better understand concurrency primitives and handson with Rust. Nothing here promises to be idiomatic Rust or even the right design.

TODOS:
- Go through https://crates.io/crates/multiqueue, https://github.com/abbychau/multiqueue2, https://github.com/zesterer/flume to better understand how this is done in real-world Rust
- Read about Futures
