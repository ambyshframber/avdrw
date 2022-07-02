# avdrw

a simple block read/write tool for [avd](https://github.com/ambyshframber/avd) archives

## usage

`avdrw drive {r|w|n} [blknum file]`

`drive` is the path to the archive file. `r` for read (avd -> host drive), `w` for write (host drive -> avd), `n` to create a new empty drive. `file` is the path to the other file involved. `blknum` is the block number, in hex. `blknum` and `file` are not required for the `n` mode.
