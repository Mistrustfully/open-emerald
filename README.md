# Open Emerald
An open source, open world Pokèmon Fangame.
Inspired largely by [PokèWilds](https://github.com/SheerSt/pokewilds).

## Licensing
All code is licensed under GPL-3.0-or-later (see [copying](COPYING)). License headers are managed by [Reuse](https://reuse.software/). 

Assets are owned by GameFreak. 

## Building
You can build the client / server with `cargo build -p client` / `cargo build -p server` respectively. The client includes an integrated server binary.

Building in debug mode (as in, without `--release`), will enable debug features like `bevy_editor_pls`.
