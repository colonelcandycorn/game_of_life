# Life
## Creator: Sarah Dylan
## What I did
I think the first step was just setting up an embedded project, making sure I had the right
crates added as dependencies and my config.toml was setup correctly. Once I was successfully 
load a blinky program on the board, I tried to write a random board function. I ended up calling
this init_board. It went through several variations because I was unsure how to initialize the random
number generator on the board. I originally was passing a reference to the board as an argument, but I realized
I needed to pass a reference to Pcg64 if I wanted to call the function multiple times.

When I started working on the next step, getting the buttons working, it was shortly after our lecture
on interrupts. I ended up basing a lot of my code on [this example](https://github.com/pdx-cs-rust-embedded/gpio-hal-printbuttons/blob/main/src/main.rs).
