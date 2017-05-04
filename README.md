# ashuffler

> This owes its existence to [ashuffle][], but is a different take on the same
> idea. Also it is part of my "lets learn Rust" set of projects, so the code may
> not be the best...

ashuffler provides an automatic shuffle for [mpd][]. It can be run in two modes:

1. Randomly queue a number of songs, then quit, by passing the `--only` flag
   with the number of songs to queue.

   ```
   $ ashuffler --only 10
   ```

   will queue 10 songs, and nothing else.

2. Run in continuous mode. In this mode ashuffler will maintain a constant
   "buffer" after the currently playing song (by default 1 song, but this can be
   changed with the `--buffer` flag).

   ```
   $ ashuffler --buffer 5
   ```

   will check if there are fewer than 5 songs after the current one, if so it
   will add random songs until there are 5.


### install

```
$ cargo install --git https://github.com/hawx/ashuffler
```

[ashuffle]: https://github.com/joshkunz/ashuffle
[mpd]: https://www.musicpd.org/
