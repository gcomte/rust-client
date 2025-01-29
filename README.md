# Rust WunderGraph client POC

Just a POC to be used as a reference for future implementations. 

## Setup

### WunderGraph server

1. Clone the [WunderGraph server docker repository](https://github.com/wundergraph/docker)
2. From within the repo, build the docker image: `docker build -t wundergraph .`
3. Launch a container: `docker run -p 9991:9991 wundergraph:latest`
4. Test whether you can retreive the dragons: `curl -X GET http://localhost:9991/operations/Dragons`

### Rust client (this project)

`cargo run --example dragons`

