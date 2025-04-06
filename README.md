# BIP 380

BIP 380, but in its fastest street-legal version.

## How to start ✨

> 💡 This manual covers setup for Linux-based operating systems. If you are planning to run this on some other platform, we suppose you already have the knowledge to do so. Nevertheless, the process is very similar.

There are two ways you can get started with this tool:

### Option 1: Download and run an release

1. 💾 Download an executable from our [release page](https://gitlab.fi.muni.cz/pv286/teams/team-15/-/releases).

2. ⚡ Open a terminal in the same folder where you downloaded the executable and run it.
    ```bash
    ./bip380 --help
    ```
    > If an error occurs while trying to do this, you may need to add a permission for executing the file. You can do that with `chmod +x bip380`.

3. 🚀 If the previous command displayed the help message, you're **good to go**.

### Option 2: Run the source yourself

> ⚠️ Note that this option is **not recommended** for users with minimal programming knowledge. You will also need the Rust toolset installed before proceeding, you can learn more about that at the [official Rust documentation page](https://www.rust-lang.org/tools/install).

1. 💾 Clone or download this repository. This can be done via `Code` dropdown section at the [repository home](https://gitlab.fi.muni.cz/pv286/teams/team-15), or by using one of the methods below.

    <details>
    <summary>SSH</summary>

    ```bash
    git clone git@gitlab.fi.muni.cz:pv286/teams/team-15.git
    ```

    </details>

    <details>
    <summary>HTTPS</summary>

    ```bash
    git clone https://gitlab.fi.muni.cz/pv286/teams/team-15.git
    ```

    </details>

    <details>
    <summary>ZIP</summary>

    🖱️ Click [this link](https://gitlab.fi.muni.cz/pv286/teams/team-15/-/archive/main/team-15-main.zip) to download. After downloading, extract the ZIP file contents.

    </details>

2. 📂 Enter the folder with the code (the name can differ if you specified a custom name).
    ```bash
    cd team-15
    ```

3. 🔨 Compile the code.
    ```bash
    cargo build --release
    ```

4. ⚡ Run the produced executable.
    ```bash
    ./target/release/bip380 --help
    ```

5. 🚀 If the previous command displayed the help message, you're **good to go**.

> 🗒️ You can also run the code without explicitly pre-compiling it with `cargo run` (more about it [here](https://doc.rust-lang.org/cargo/commands/cargo-run.html)).

### Testing 🧪

You can run all the tests for the project with this command:

```bash
cargo test
```

> As noted in the libraries section below, some of the tests require that you build the application (i.e., running `cargo build`) prior to running the tests.

## Used libraries 📚

The base cryptographic library is `bip32`. [🔗](https://docs.rs/bip32/latest/bip32/)

For base58 encoding, our project utilizes the `bs58` crate. [🔗](https://docs.rs/bs58/latest/bs58/)

For the testing purposes, there is a `assert_cmd` library that simulates running this tool from the command line. [🔗](https://docs.rs/assert_cmd/latest/assert_cmd/) 
> The **assert_cmd** also requires that you build the application (i.e., `cargo build`) prior to running any test that uses this library.

To compute SHA-256 hashes, we rely on the `Sha256::digest` that is imported from the `bip32` crate. 