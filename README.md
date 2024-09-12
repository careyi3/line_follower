# Line Follower

Line following robot control system on an STM32L432 written in Rust.

## Set Up

For setup I am assuming you are using an unix or linux system with Rust already installed.

Run the below commands to install the recessary cargo tools:

```bash
rustup update
rustup component add llvm-tools-preview
rustup target add thumbv7em-none-eabihf
cargo install cargo-binutils cargo-embed cargo-flash cargo-expand
```

## Chip Specific Config

You will find several places in this code base where you need to specify the chip you are using. In my project I use an STM32L432KB. You can find this referecnes in a few places, you will need to update this to be the chip you are using if you aren't using the same. You will also need to update the hal libary being used in `Cargo.toml` to match your chip. You should be able to find supported chip HAL libararies [here](https://github.com/stm32-rs).

You will also need to modify the `memory.x` file to specify the memory location for FLASH and RAM on your chip and the length of these registers. This information can be found in the datasheet for your chip.

## VSCode

To debug using VSCode you will need to install the [probe-rs software](https://probe.rs/docs/getting-started/installation/) locally. You can do this using the below:

```bash
brew tap probe-rs/probe-rs
brew install probe-rs
```

or

```bash
cargo binstall probe-rs
```

After this you will also need to install the [extension for VSCode](https://marketplace.visualstudio.com/items?itemName=probe-rs.probe-rs-debugger) and follow their [setup guide](https://probe.rs/docs/tools/debugger/#usage-and-configuration).

## Running

Once connected to an STLink programmer, you can run the code on the chip and view debug logs by running:

```bash
cargo embed
```

If you have configured VSCode for debugging you can execute your code in the IDE and step through it with break points etc.
