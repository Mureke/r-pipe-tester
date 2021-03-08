## R-pipe-tester
Small test app for creating named pipes and testing them.
#### How to build:
`$ cargo build --release`

### How to run:
`$ cargo run <is_testing>`
 * is_testing = "true" - spawns second thread that writes to named pipe and reads the response.