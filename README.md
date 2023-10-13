# broadcast-rs


***Disclaimer**: This is an assignment for the Empowered Programmer Cohort. An attempt to better understand concurrency primitives and handson with Rust. Nothing here promises to be idiomatic Rust or even the right design.*

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

Sample Output:
```bash
rust_routine_1: message from 0 from rust_routine_0
rust_routine_4: message from 0 from rust_routine_0
rust_routine_3: message from 0 from rust_routine_0
rust_routine_2: message from 0 from rust_routine_0
rust_routine_2: message from 4 from rust_routine_4
rust_routine_0: message from 4 from rust_routine_4
rust_routine_1: message from 4 from rust_routine_4
rust_routine_3: message from 4 from rust_routine_4
rust_routine_0: message from 1 from rust_routine_1
rust_routine_4: message from 1 from rust_routine_1
rust_routine_3: message from 1 from rust_routine_1
rust_routine_2: message from 1 from rust_routine_1
rust_routine_0: message from 2 from rust_routine_2
rust_routine_1: message from 2 from rust_routine_2
rust_routine_4: message from 2 from rust_routine_2
rust_routine_3: message from 2 from rust_routine_2
rust_routine_0: message from 4 from rust_routine_4
rust_routine_3: message from 4 from rust_routine_4
rust_routine_2: message from 4 from rust_routine_4
rust_routine_1: message from 4 from rust_routine_4
rust_routine_0: message from 3 from rust_routine_3
rust_routine_2: message from 3 from rust_routine_3
rust_routine_1: message from 3 from rust_routine_3
rust_routine_4: message from 3 from rust_routine_3
rust_routine_0: message from 2 from rust_routine_2
rust_routine_4: message from 2 from rust_routine_2
rust_routine_1: message from 2 from rust_routine_2
rust_routine_3: message from 2 from rust_routine_2
rust_routine_1: message from 0 from rust_routine_0
rust_routine_4: message from 0 from rust_routine_0
rust_routine_2: message from 0 from rust_routine_0
rust_routine_3: message from 0 from rust_routine_0
```


TODOS:
- Go through https://crates.io/crates/multiqueue, https://github.com/abbychau/multiqueue2, https://github.com/zesterer/flume to better understand how this is done in real-world Rust
- Read about Futures
