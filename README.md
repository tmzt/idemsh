
The Idempotent Shell
=================

## Goals:

* simple, declarative commands
* easy remote access and support for enumerating remote systems
* access powerful URI paths, including remote cloud storage such as S3

Single static binary, which is also used to perform commands on remote systems when scp, sftp, etc. are not enough.

Built-in config file editing syntax, with smart detection for blocks.

## Getting Started

`idemsh` is distributed as a single static binary, but can load external plugins if available.

Let's examine the syntax:

    ./path1 ./path2 (copied)

This will copy the contents of ./path1 into ./path2, unless that copy has already occurred. This is done by quickly hashing and check the contents of the destination file. If ./path1 is a directory, this command will fail because the `-r` or `recurse` flags were not given.

Paths can be a simple path, a glob, a path with a `**` pattern in it (similar to minimatch), a remote path with the format used by scp, or a URI.

One or many paths can be provided, depending on the command.

Blocks are simple, starting with a _block command_, such as `each` and ending with `end`.

## Planning

This program is a work in progress, see the [PLANNING.md](PLANNING) document for what are doing. `idemsh` follows symantic versioning, pre 1.0 releases may break compatbility with previous versions, though this will be kept to a minimum.
