# avdrw

a simple block read/write tool for [avd](https://github.com/ambyshframber/avd) archives

## usage

`avdrw drive blknum {r|w} file`

`drive` is the path to the archive file. `blknum` is the block number, in hex. `r` for read (avd -> host drive), `w` for write (host drive -> avd). `file` is the path to the other file involved.
