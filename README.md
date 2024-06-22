# memory-game-rs

Implementation of a simple memory game. Cards with different symbols are randomly place in a rectangular matrix (hidden, player sees backside of the card). Players pick two cards. If the cards match, the player keeps the cards. Otherwise they go back. The player with most collected cards wins.

## Why?

This is just to learn a bit of rust.

So far, I find the language... annoying. This borrow checker thing is just too difficult for simpletons like me. Also, the syntax is hard to memorize. And why are they ditching OO concepts such as inheritance? These are not bad ideas, and you don't have to use them when you don't like them...

Lifetimes are not manageable for normal human beings. When I hear rust experts say, they involve an AI when they don't know how to overcome a borrow checker issue, that tells me, that there is somethign wrong with the language.

So why keep on doing it? Stubborness :-) I started it and I want to finish it!

## Status

Elementary game logic is implemented. I intend to use sdl2 for a simple UI. Maybe on the long run I also try to make this multi-player working over the network. I might even invent an "AI player" if you play alone.

## Notes

I originally planned to build with gtk but changed my mind and went for the much simpler and more game oriented SDL2.

## How to build

Just clone the project and build with cargo:

```bash

cargo run

```

You will have to install the SDL2 and SDL2::ttf libraries on your computer. On Mac you might have to set the LIBRARY_PATH environment
variable to find the SDL2 all right. Otherwise I used brew install to get the libaries installed on my Mac, but it is a bit tricky.

Have fun.
