# S H I P  G E N

Generates spaceships on the command line. Benny would love it.

This is a simple little Rust app to get my feet wet with the language as I work through the official rust book.

## Usage

```shell
./ship_gen --help
rocket 

USAGE:
    ship_gen [OPTIONS] --height <HEIGHT>

OPTIONS:
    -h, --height <HEIGHT>      
        --help                 Print help information
    -p, --palette <PALETTE>    [default: america]
```

Running it spits out ships on stdout, like:
```shell
./ship_gen --height 20
    │
    ║
   /'\
   │ │
  /│ │\
   │ │
   │°│
   │°│
   │°│
  ┌┘ └┐
  │   │
  │ O │
  │   │
  │   │
  │° °│
 /│ ^ │\
/_│ | │_\
   \_/
   ( )
    ·
```

## TODO

 * Implement color palettes
