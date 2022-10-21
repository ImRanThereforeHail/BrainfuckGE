# Simple Brainfuck Game Engine

An engine where the logic is written in brainfuck. 

# Notice
Due to the language's specifics **the only available colors are black and white**.
A value may go as high as **255**, but for the canvas, the only accepted values are **0 - black** and **>1 - white**.

This is a feature, not a bug.

Also, in order to get input, you use ',LetterOfYourChoice', for instance

```
,w
```
In this code, if 'w' was clicked between globs (aka if 'w' was clicked between canvas update) the selected value will become 'w''s ascii code.

# Setup

Here is what you need to do to use this engine.

```bash
git clone https://github.com/ImRanThereforeHail/BrainfuckGE.git
cd BrainfuckGe
cargo run --release main.bf
```
A "Hello world!" message should appear. 

# First steps
Head on to any file of your choice

The inner workings of the engine are very simple. You have an array of size **1024** which acts as a **32 x 32 grid** in the canvas. (In future versions you should be able to adjust the size). If a value inside the array is greater than 0 its corresponding pixel will be white; else, the corresponding pixel will be black.

The brainfuck code runs in a loop; 

# Answering questions

- Can I have objects?
You can! So long as said object isn't invisible (which is not supported) just assign a number from 0 to 255 as an alias to said object, and in the next iteration of the loop you'll know what pixel represents what.

# Examples 

This code paints the whole canvas white
```brainfuck
+>*
```
This is because the code runs in a loop, so, in english:

```
add 1 to the current value, change to the value in the right, update screen, return to the beginning 
```
Thus, if you want to paint, for instance, 2 cells, it would have to be:
```
+>+*-<-
```


# TODO
- Be able to change grid size (CAN'T DUE TO UNKNOWN GLITCH)

# Done
- File to be used goes an argument, rather than always the main.bf
- Use '*' to update canvas
- Rewrite front-end (all in src/main.rs)