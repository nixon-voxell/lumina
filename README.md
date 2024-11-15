# Lumina

A top down fast paced objective based PvPvE game.

## Quickstart

To compile Lumina, you have to perform a recursive clone:

```
git clone --recursive https://github.com/nixon-voxell/lumina.git
```

### Run the game

Before running the game, the `assets` folder needs to be linked correctly to all the binary crates.
You can do so by running:

```
create_asset_junctions.bat
```

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

To improve development time, the `crates/test_bed/examples` folder is used to create mini test cases with minimal compilation time. Run a test example using the following command:

```
cargo run --example test_name
```

With `test_name` being the name of your testing example.
