
## [IPC Explained: When Your Processes Need to Talk or Yell at Each Other](https://medium.com/@lordmoma/ipc-explained-when-your-processes-need-to-talk-or-yell-at-each-other-9ca29a515e99)

### How to run this project

```bash

cargo build

# To run the shared memory parent process
cargo run --bin shared_memory_parent

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/shared_memory_parent`

Starting shared memory parent process
Parent incremented counter to: 1
Starting shared memory child process
Parent incremented counter to: 2
Child incremented counter to: 3
Parent incremented counter to: 4
Parent incremented counter to: 5
Child incremented counter to: 6
Parent incremented counter to: 7
Child incremented counter to: 8
Child incremented counter to: 9
Child incremented counter to: 10
Final counter value: 10


# To run the message passing parent process
cargo run --bin message_passing_parent

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/message_passing_parent`

Starting message passing parent process
Starting message passing child process
Child sending increment request 1
Parent received increment request, counter: 1
Child received counter value: 1
Child sending increment request 2
Parent received increment request, counter: 2
Child received counter value: 2
Child sending increment request 3
Parent received increment request, counter: 3
Child received counter value: 3
Child sending increment request 4
Parent received increment request, counter: 4
Child received counter value: 4
Child sending increment request 5
Parent received increment request, counter: 5
Child received counter value: 5
Parent received get value request, counter: 5
Final counter value: 5
Child disconnected


```
