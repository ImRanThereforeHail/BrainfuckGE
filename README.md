# Simple Brainfuck Game Engine

An engine where the logic is written in brainfuck. 

# Notice
## Rendering
Due to the language's specifics **the only available colors are black and white**.
A value may go as high as **255**, but for the canvas, the only accepted values are **0 - black** and **>1 - white**.

This is a feature, not a bug.

## Language

The engine runs on a custom brainfuck, tailored for gaming. Below are the changes:

### Headers

Headers are always in the beginning of a file, separated by '£'. A .bf file used by this engine should start with something like:

```
64£32£*£
```
Let me go over this.

The format is "x £ y £ init_script *£ code"

A better explanation will come out tomorrow

### Loops

Should be mentioned that **loops are not do-while**, they are **while**. this means that something like

```
[+>]*
```
Won't show anything

### Comma (,)

In order to get input, rather than using just ',', you use ',LetterOfYourChoice', for instance

```
,w
```
This was decided as opposed to using just ',' as that was too hardcore, even for me. Keep in mind that in the above snippet **if w was not pressed, the code will not hang, and nothing will happen**. This means that, for instance, in the following code:

```
>,a[[>]+[<]>-]*<
```
Blocks which number as many as a's unicode will spawn only when you press a, without waiting for a newline.


### Glob (*)

The glob was added to refresh the screen. Whenever the glob appears in the code, the screen is refreshed! You can use it multiple times. For instance, in the code:

```
+*->+*[-<]
```

You should now see 2 blocks blinking!

Note: The init script should **always** end with a glob (if you want it empty, use only a glob)

### Back slash (\\)

In BrainFuckGE there is a second array, since without it, all the data you have is transmitted to the screen. The second array is used for *under the hood* stuff, and \\ toggles between the main array and the subrarray. Think of it as the inventory toggle in *Stardew Valley*!

It's hard to give a simple example for this one, here goes!

```
Add 4 to index 0 in the subarray
\++++ 

Nothing will appear
*

Move the value (in this case 4) to a cell in the main array
[\+\-]

Now a white block should appear
*

The next line undoes what we just did
\[-]

Now the white block should be blinking 
```

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

This code paints a whole 32x32 canvas white
```brainfuck
32£32£*£
+>*
```
This is because the code runs in a loop, so, in english:

```
Create a canvas of size 32x32, show it, add 1 to the current value, change to the value in the right, update screen, return to the beginning 
```
Thus, if you want to paint, for instance, 2 cells only, it would have to be:
```
+>+*-<-
```
Here, we paint two cells, show the screen, then undo what we did. After that, it runs cyclically until you stop it!



# TODO
- Fix smaller resolution glitches

# Done
- File to be used goes an argument, rather than always the main.bf
- Use '*' to update canvas
- Rewrite front-end (all in src/main.rs)
- Fixed main resolution glitch
