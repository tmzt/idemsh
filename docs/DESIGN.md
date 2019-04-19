
Path formats
===========

First, paths are normally URLs, but there are some exceptions.

Exceptions
--------------

### `file:///`

Any path will be treated as a file if it does not fall under one of the other rules

### `ssh://`

Any path including `@` and `:` will be treated as `ssh://` unless another scheme is used.

Hosts
--------

Hosts are taken from multiple sources, and can include groups which are both automatically created by plugins and read from the configuration in both the current directory (or repository) and the HOME directory.









