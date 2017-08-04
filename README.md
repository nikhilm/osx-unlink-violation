TODO: good program name, not foo
# POSIX unlink/open violation on Mac OS

Open a file using O_EXLOCK, unlink the file, close the FD.
Turns out there is a small window of time, on the order of milliseconds, where
another process can sneak in and open the same file, after the close. This is
even though the link count has dropped to zero. A dtrace probe shows the
sequence of calls.

Things to try:
- Does this happen on other Unix?
- Does this happen if not holding an EXLOCK?

## Instructions

You will need rust and cargo. First run

    cargo build

The gen.py program generates a set of sample files for the rust program to act
on. A good way to run this to make the problem occur is:

    python gen.py 50; ./target/debug/foo&; ./target/debug/foo

The problem occurs infrequently. The tell-tale is the `!!!...` prefixed call to
unlink that results in an error, indicating the second process was able to open
the file, but not unlink it. The output has the process PID as the first line.

The openclosesnoop dtrace script is a modification of Brendan Gregg's opensnoop
that ships with OSX. It probes close and unlink in addition to open. Run as:

    sudo ./openclosesnoop -a -g -F -n foo

before starting foo to see the activity indicating the violation.

Full details in the blog post TODO.
