# stakedex-cli

Public-facing CLI for the stakedex solana program

## Usage

`stakedex --help` to read available commands and details.

## Cross-compilation

`Dockerfile`s are provided to make cross-compilation easier. Use `docker cp` to extract the binary.

```
docker run --name stakedex-cli stakedex-cli; docker cp stakedex-cli:/stakedex stakedex && docker rm stakedex-cli 
```

### x86_64-unknown-linux-musl

`docker build -f Dockerfile.x86_64-linux-musl -t stakedex-cli .`
