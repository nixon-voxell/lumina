# Lumina

A top down fast paced objective based PvPvE game.

## Quickstart

To compile Lumina, you have to perform a recursive clone:

```
git clone --recursive https://github.com/nixon-voxell/lumina.git
```

### Run the game

To run the game, you need to start the server and the client.
You can do so manually using:

```
cargo run --bin lumina_server
cargo run --bin lumina_client
```

For development purposes, a shell script has been created to speed things up:

#### Windows

```
run.bat x
```

With `x` being the number of clients you want to spawn.

### Test Bed

To improve development time, the `crates/test_bed` folder is used to create mini test cases with minimal compilation time. Run a test binary using the following command:

```
cargo run --bin test_name
```

With `test_name` being the name of your testing binary.
