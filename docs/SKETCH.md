
```

./directory (exists)
./source ./target (copied)
./source ./target (cp)

./directory (removed)
./directory (removed -r)
./directory (removed recurse)

#
# For each directory in the directory
#

each dir i in ./directory
    $i/file1 (touch)
    $i/file2 (dirs, content = "")
    $i/file3 (dirs, content = """
        The contents of {{ i }}
""")
end

#
# Find, using ** in paths
#
list ./**
echo i in ./**/*.txt

#
# Editing a file
#

/etc/passwd (edit = {
    $+ new line to add to file (/^new line/)

    @#Existing line to change# s/change/modify/g
})

#
# Remote
#

remote <<host>>
end

remotes <<hosts or groups>>
end

remotes host in <<hosts or groups>>
    # Copy back to most recent scope
    ./remote-file : (copy)

    # Copy from most recent scope to remote
    :./local-file ./remote-file
end

remotes host in <<hosts or groups>> using bash
    for i in *.txt; do
        echo "Remote file: ${i}"
    done
end

remotes host in <<hosts or groups>> as user:group
    echo $(id)
end
