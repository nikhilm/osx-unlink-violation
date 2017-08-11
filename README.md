# Interesting file locking and unlink interactions on OSX.

See the blog post for details.

## Build instructions

You will need [Rust](https://rust-lang.org) and cargo. Best to use rustup.
Clone this repo, then run

    cargo build

This will leave the binary as `target/debug/osx-unlink-violation`.

The `openclosesnoop` DTrace script is a modification of Brendan Gregg's opensnoop
that ships with OSX. It probes close and unlink in addition to open. Run as:

    sudo ./openclosesnoop -a -g -F -n osx-unlink-violation

to start monitoring system calls. If you are on Sierra or higher, you will have
to [enable DTrace](http://blog.shalman.org/enabling-dtrace-on-macos-sierra/).

The gen.py program generates a set of sample files for the Rust program to act
on. A good way to run this to make the problem occur is:

    python gen.py 20; ./target/debug/osx-unlink-violation&; ./target/debug/osx-unlink-violation

The problem occurs infrequently. The tell-tale is the `!!!...` prefixed call to
unlink that results in an error, indicating the second process was able to open
the file, but not unlink it. The output has the process PID as the first line.
