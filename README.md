# Simple Brainfuck Game Engine

An engine where the logic is written in brainfuck. 

# Notice
Due to the language's specifics **the only available colors are black and white**.
A value may go as high as **255**, but for the canvas, the only accepted values are **0 - black** and **>1 - white**.

This is a feature, not a bug.

# Setup

Here is what you need to do to use this engine.

```bash
git clone 
cd brainfck
cargo run --release
```
A "Hello world!" message should appear. 

# First steps
Head on to (as of now) main.bf (In future versions, you should be able to use any file of your choice)

The inner workings of the engine are very simple. You have an array of size **1024** which acts as a **32 x 32 grid** in the canvas. (In future versions you should be able to adjust the size). If a value inside the array is greater than 0 its corresponding pixel will be white; else, the corresponding pixel will be black.

Every 1/60 of a second, the code is compiled and the canvas is updated. **The array used to construct the canvas is never deleted**. This means that you can build your program through seeing the last frame. This is **extremely simple and limiting**, HOWEVER this engine is not expected to take on any projects bigger than a snake game.

# Answering questions

- Can I have objects?
You can! So long as said object isn't invisible (which is not supported) just assign a number from 0 to 255 as an alias to said object, and in the next iteration of the loop you'll know what pixel represents what.

# Examples 

This half assed boilerplate code paints half the canvas of white. As you can see, each "+>" represents a pixel
You'll usually want to use loops though
```brainfuck
+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>
+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>
+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>
+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>
+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>
+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>
+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>
+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>
+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>
+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>
+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>
+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>
+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>
+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>
+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>
+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+>+
```
Keep in mind that since the array is never deleted, the values will keep going up until they reach 255.




# TODO
- File to be used goes an argument, rather than always the main.bf
- Be able to change grid size