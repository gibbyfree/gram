# Gram

Gram is a Rust-based implementation of [kilo](https://github.com/antirez/kilo), a simple text editor. I loosely followed [this guide](https://viewsourcecode.org/snaptoken/kilo/) during development.

Usage: `./gram <filename>` 
Example: `./gram sample.c`

Build for your target with `cargo run` or `cargo build`.

Keyboard shortcuts:
```
CTRL-S: Save
CTRL-Q: Quit
CTRL-F: Find string in file (ESC to exit search, arrows to navigate)
```

```
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 C                       1           28           18            4            6
 Markdown                1           13            0           10            3
 Rust                   14         1897         1410          299          188
 TOML                    1           12            9            1            2
===============================================================================
 Total                  17         1950         1437          314          199
===============================================================================
```

Er, 'simple'... right... 😅