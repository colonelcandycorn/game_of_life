# Life
## Creator: Sarah Dylan
## What I did
I think the first step was just setting up an embedded project, making sure I had the right
crates added as dependencies and my config.toml was setup correctly. Once I was successfully able to
load a blinky program on the board, I tried to write a random board function. I ended up calling
this init_board. It went through several variations because I was unsure how to initialize the random
number generator on the board. I originally was passing a reference to the board as an argument, but I realized
I needed to pass a reference to Pcg64 if I wanted to call the function multiple times.

When I started working on the next step, getting the buttons working, it was shortly after our lecture
on interrupts. I ended up basing a lot of my code on [this example](https://github.com/pdx-cs-rust-embedded/gpio-hal-printbuttons/blob/main/src/main.rs). I changed the channel setup for the a button to be based on toggle instead. I also setup two global variables to test whether the buttons have been pressed. 

From there, the only real challenge left was to write a complement board function and piece everything together.

## How it went

The linker errors were particularly annoying as I couldn't really figure out why it wasn't working, and I don't know why adding the TLink.x fixes the error. 

I also realized I wasn't a 100% sure how to actually read the value of the pin instead of relying on a variable I toggle manually.

The code also for the interrupt seems quite mystical, and I don't quite get what is going on in some of the setup.

I think I did a good job of piecing things together, but I feel like I need to spend more time understanding how the code works. Because at this point, I feel pretty reliant on the example code in the class repository. 
