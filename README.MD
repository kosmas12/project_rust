# Project Rust

Project Rust is a text-based game written in Rust.
I use it as an opportunity to learn Rust and combine
along with it the thing that made me want to start
programming in the first place: Game Development.

This isn't close to a game that a big company would
make, but I'm trying my best to make it good in regard
to the story and the gameplay. Don't want you to get
bored, now, do I? :)

# License

This code is licensed under the GNU General Public
License Version 2. This allows you to freely make your
own copies of the source code, modify them and
distribute them, as long as you follow certain rules.
See COPYING for details.

# Getting the code and compiling

If you would like to compile the source code on your
own instead of using the pre-compiled builds for your
operating system, then it's very easy to do so, thanks
to Rust's way of compiling projects. (At least for
now,) Project Rust requires nothing but the Rust
1.9.0 (or newer) standard library and its build
system, `cargo`, that comes preinstalled with Rust.

To clone the repository you will need to install Git.
After you install it, run `git clone` and next to
`clone` put the URL of the repository that you can
get by clicking the green code button on GitHub
(don't use the one on your browser's bar, that's
different). 

After you get the pre-requisites and have cloned
the repository, compile by entering the directory
where you cloned it and run the following command:

`cargo build --release`

In the `target/release` directory there should now be
the Project Rust binary. You can now play it.


# Launching the game

Project Rust is a simple text-based game, so the only
thing it needs to run is a command line or terminal.
To run it, go into the `target/release` directory 
and double click it, or enter the directory with your
terminal and run:

For Linux, Mac, and any UNIX-like OS:
`./project_rust`

For Windows: `project_rust.exe`
