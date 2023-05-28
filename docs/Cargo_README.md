## Cargo

1. Creates a new crate:

   `cargo new <name>`

 
1. Build the crate:

   `cargo build`

   `cargo b`

1. Run the crate:

   `cargo run`

   `cargo r`

1. Silently run:

   `cargo r -q`

1. Check whether your program can compile:

   `cargo check`

   `cargo c`

   People use this command to periodically check their programs. After working on their program, and when they're sure, they build your program using `cargo build --release`.

   This command produces a faster program, but it'll take a longer time to compile.

   This is also the command you want to use when you benchmark your code.

1. Cargo caches. If you want to start on a clean slate, run:

   `cargo clean`

   Sometimes, `cargo check` returns a warning. However, when you run it the second time, it doesn't display the warning. In that case, you can run `cargo clean`, and then run `cargo check` again. This way, you'll be able to see the error message again.

1. You can see the documentation of every crate that your program/package depends on.

   `cargo doc --open`

1. Create a library package.

   `cargo new --lib <name>`
   
   _This command will create a package with a test._
