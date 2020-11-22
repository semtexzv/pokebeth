## Shakespeare pokemon api
Returns shakespearean description of a pokemon. Usage:
1. Start the application by running `cargo run`
2. Call the api by running `./translate.sh <optional name>`


### Dependencies
Probably too many due to 2 different async runtimes.

[tide](https://crates.io/crates/tide) - Webserver

[reqwest](https://crates.io/crates/reqwest) - Client

[anyhow](https://crates.io/crates/anyhow) - Error management

[serde](https://crates.io/crates/serde) - De/Serializiation
 
### Requirements
Rust + Cargo installed. If needed, follow instructions [here](https://www.rust-lang.org/)
