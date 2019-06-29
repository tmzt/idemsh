
Planned Features
==============

These are the features we are planning. As they are implemented, they will be moved into the README.

## Remote access

One of the main features of `idemsh` is it's simple integration of remote access, including to HTTP/HTTPS/FTP/SFTP, cloud storage such as S3, and any machine reachable via ssh.

When accessing remote machines, one of two modes is used. The first is a combination of ssh (with persistent connection) and scp (sftp if available).

The second is unique to `idemsh` and involves installing `idemsh` on the remote machine. This will be done if the `remote` `prepare` command is given:

    remote <path> (prepare)

The first time `idemsh` is installed on the remote machine, it will ask for confirmation. 

Once installed, `idemsh` will communicate with the remote machine using it's own binary protocol, enabling instant use of all `idemsh` features and fast file transfers and remote directory mounts.

## Cloud Hosts

If a cloud plugin is loaded and enabled, `idemsh` will enumerate the machines which are accessible using the default credentials.

For instance:

    ec2:// (enable-plugin)

Will make all the instances accessible through the default profile available as hosts with the syntax i-<instance_id>. They can also be referenced explicitly with the `ec2://` URI scheme.

This scheme supports other queries, such as `ec2://instances?tag-<tag>=<value>` and `ec2://<name>` where `name` is the `Name` tag assigned to the instance.

Plugins can also be enabled by giving the path to the `.so` file containing the plugin.

## Cloud Storage

In addition the the built-in protocols (http/https/ftp/ftps/sftp/ssh), plugins enable support for services such as S3 with the `s3://` URI syntax. This has the same format as the `AWS` cli, `s3://<bucket>/key`. An alternative `https+s3` protocol is also supported, which takes the full hostname of the S3 endpoint, like `https+s3://s3.amazonaws.com/<bucket>/<key>.

## Remotes and Bastions

Complex arrangements for access remote hosts are supported, including so called bastion hosts, where access to one machine is done by tunneling through another machine, or even by invoking `ssh` within an `ssh` session.

This is supported using nested `remote` blocks with the `(keep)` flag, such as:

    remote bastion1 with
       remote host1 (keep)
    end

This can be added to your `~/.idemshrc` file.

Access to `host1` will then automatically use a connection to `bastion1`, and will use a persistent connection if one is available.

## The hosts:// URI

All hosts known at any time to `idemsh` can be accessed explictly using the `hosts://` URI syntax. For instance, `hosts://prod-*` will refer to all *known* host names starting with `prod-`.

This URI can also be enumerated using the `each` command or listed with the `list` or `ls` keyword flag.

    each h in hosts://prod-*
       $h (deploy-software)
    end

This will execute the `deploy-software` command on each host matching `prod-*`.

## Defining commands

Commands can be added to `idemsh` using the following syntax:

    def command<(params)>
        <statment ...>
    end

The `params` can be given one of two ways, as positional parameters:

   def command(pos1, pos2, ...) ...

Or as keyword params, which have the syntax:

    def command({ arg1, arg2 }) ...

Both syntaxes support default parameters using `=`:

    def command( pos1 = "default" ) ...
    def command({ arg1 = "default" }) ...

The keyword parameter syntax also supports "splatting" using the leading `...` (elipses):

    def command({ ...args })

Keyword flags are only supported in the keyword parameter form, and are given as matching alternatives using parens and the `|`:

    def command({ (recurse|-r) = false }) ...

## Calling custom commands

Calling a custom command follows the normal paths syntax, with the command placed before the parameters and keyword flags:

    ./path (command pos1, pos2, ...)

If using keyword params, the commas are omitted:

    ./path (command arg1="x" arg2="y")

If no value is given, the argument is treated as a true boolean keyword parameter.

## Custom block commands

Custom block commands are defined differently:

    defblock command [word1 "keyword" word2] (<args>) do
    end

The second set of arguments follows the same syntax as a normal custom command, the first is always positional. The first set of arguments can also contain quoted strings which are treated as tags to be recognized. 

If a block is not expected, the `do` statement can be omitted, and the `end` can directly follow the arguments, but the arguments are mandatory.

## Invoking custom block commands

Custom block commands are invoked like built in commands:

    command word1 keyword word2
    end

And with arguments:

    command word1 keyword word2 (<args>)
    end

If arguments are provided, the block can be ommitted, and normal newline/indentation rules will be followed.

## Commands as expressions

Commands can be invoked as expressions using the double paren syntax:

    (( ./file (wc) ))

This will count the words in the file and return the result. In custom commands and block commands, the `return` keyword is used to explictly return from the block with a result. The last value in the block is also treated as the return value of the block.

## Success and error

A successful command will return the value of the command, or the empty value `(())`.  A failed command will return a Failure result. If an external command is invoked, the result code wil be translated following normal unix conventions, non-zero results will be treated as Failure(ResultCode(<code>)).

## `with` blocks

The `with` keyword is supported as a block command and will set the default context for values in the block. It can also be used to ensure a resource for the scope of the block, such as a temporary file.

For instance, to create a temporary directory with a suffix using the `mktemp` facility.

    with <host> as remote
        with (( temporary(dir suffix="build") )) as d
            {{d}}/test1 (contents = "Temporary file in {{ d }}")
            system (make)
            {{d}}/a.out {{remote}}:app
        end
    end

The temporary directory will only exist for the scope of that `with` block, and will be deleted (recursively) when the statements contained within complete.

Performing a `with` on certain types, such as `Directory` or `Host` without the `as` clause will automatically switch the context to that directory or host:

    with <host>
        with (( temporary(dir suffix="build") ))
            ./test1 (contents = "Temporary file in {{ (pwd) }}")
            system (make)
            ./a.out .:app (copy)
        end
    end

As a special syntax for temporary directories and files, the double tilde (`~~`) can be used. Other shortcuts are used here to illustrate brevity in `idemsh`.

    with <host>
        with ~~build/
            ./test1 (contents = "Temporary file in $(pwd)")
            $((make))
            ./.a.out .:app (copy)
        end
    end

If a trailing slash (`/`) is provided, the temporary object is a directory, otherwise it's a file.

The `$((external))` syntax is used to invoke external commands and return a result code (wrapped as Success or Failure), but not capture the output of the command.

## Editing files and configuration template syntax

Existing files can be thought of as a collection of lines, or as a collection of blocks separated by blank lines. We'll call the first line-oriented and the second block-oriented.

Think of an editing context as similar to a `diff`, lines are added and removed using `+` and `-` symbols as the first non-whitespace character. 

We try to be more lenient than the standard `diff` algorithm, and are better tuned to working with configuration files which usually have a simple syntax.

As an example, let's look at a `yaml` file with two existing blocks.

    [main]
    config1 = "value"
    config2 = "value"

    [blockb]
    b1 = "value"
    b2 = "value"

We want to add a `[blocka]` right between those two blocks.

We could template out the whole file, but then we'd be responsible for the entire contents in our `idemsh` script.

Instead, let's use the `edit context`.

    with /etc/appconfig.yml (edit)
        [blocka]
        a1 = "value"
        a2 = "value"

        [blockb]
    end

This is a simple syntax enabled by the block-oriented nature of the file we are editing.

It works as follows:
    * A file is considered block-oriented if there are one or more blocks of the text separated by white space.
    * The file must contain a reasonable number of blocks, as the heuristic is designed to give up if the search would be  exceedingly
    complex.
    * The first line of each block in the file and in the edit context template  are hashed and stored in a data structure
    * The heuristic will attempt to match up the blocks and use it as an insertion point for the newly defined blocks.
    * If no new blocks are found, the heuristic will attempt to merge existing blocks that are more than one line, such as `blocka` in the example above. The new values will be inserted above the existing ones. This may result in duplications.
    * Special versions may be used if the syntax of the file is supported, such as for `yaml` and `toml`, in which case the value of existing keys is overwritten by new values and duplicates are prevented.

This works well if the syntax defines a name or other unique keyword at the beginning of each block, as many configuration syntaxes do.

If the support is enabled, `json` files are treated specially:

    with /etc/appconfig.json (edit)
        @/path/to/object
        {
            "a4": "value",
            "a5": "value"
        }
    end

Blocks in this format are merged into the existing JSON objects and the file is saved. An attempt is made to preserve the formatting of the file, if any. This will only work if the formatting is regular, such as a fixed number of spaces and/or newlines after an opening brace (`{`).

A special set of commands are provided for working with files in known formats, such as `json`, `csv` or line-oriented text files. ( `yaml` files are treated as `json` for the purpose of this tool)

This syntax can be used to extract data using a simplified JSON path, line-oriented regex and other similar means.

As an example using a JSON path:

    each @/parent/child[*] as c in ./test.json
        echo "Child is {{ c.name }}"
    end

Or with a line-oriented regex:

    each @~/name=(.*)/ as c in test.txt
        echo "Child is {{ c[1] }}"
    end

