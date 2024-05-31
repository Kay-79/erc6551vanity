# erc6551vanity

`erc6551vanity` is a [Rust](https://www.rust-lang.org) implementation of the profanity tokenbound account (ERC6551)

## INSTALLATION

1. Install Rust

-   ```shell
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. Install `erc6551vanity`

-   ```shell
    git clone https://github.com/Kay-79/erc6551vanity.git
    ```
-   ```shell
    cd erc6551vanity
    ```

3. Build

-   ```shell
    cargo build --release
    ```

## USAGE

-   ```Shell
    cargo run --release <registryAddress> <implementAddress> <chainId> <nftAddress> <tokenId>
    ```

-   ```shell
    cargo run --release 0x000000006551c19487814612e58FE06813775758 0x41C8f39463A868d3A88af00cd0fe7102F30E44eC 0x0000000000000000000000000000000000000000000000000000000000000001 0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D 0x0000000000000000000000000000000000000000000000000000000000000001
    ```
