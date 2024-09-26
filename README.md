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
cargo run -- server
cargo run -- client
```

For development purposes, a shell script has been created to speed things up:

#### Windows

```
run.bat x
```

With `x` being the number of clients you want to spawn.
