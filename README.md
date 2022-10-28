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

Headers are always in the beginning of a file, separated by '£'. A brainfuck source file used by this engine should start with something like:

```
32£32£*£
```
Let me go over this.

The format is "x £ y £ init_script *£ code"

You can think of the x and the y as the resolution you're playing at, being x the horizontal resolution and y the vertical. You needn't worry all that much; as of now they must be equal, or some weird glitches happen.

The init script is ran only once in the beginning of the game. It's supposed to be used, for instance, in a snake game, to spawn the snake. It must always end in * and then the usual separator, or the game hangs. Keep in mind that if you use * in the middle of the code, anything until the £ won't run.

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

You should see 2 blocks blinking!

Note: The init script should **always** end with a glob (if you want it empty, use only a glob)

### Back slash (\\)

In BrainFuckGE there is a second array, since without it, all the data you have is transmitted to the screen. The second array is used for *under the hood* stuff, and \\ toggles between the main array and the subrarray. Think of it as the inventory toggle in *Stardew Valley*!

Here's an example:

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

```
Now the white block should be blinking 

### Continuous execution

In BrainfuckGE, there are two brainfuck scripts:
- The init script
- The script

The init script is only ran once, before the first frame is rendered. The script, however, is ran in an infinite loop. **Whenever a glob appears, the frame is rendered and the code keeps running as if it isn't there**.
If there are no globs in the code, the screen will hang, as the frames won't get rendered. The same happens for infinite loops, for instance:

```
+[]*
```
Will hang.

# Setup

Here is what you need to do to use this engine.

```bash
git clone https://github.com/ImRanThereforeHail/BrainfuckGE.git
cd BrainfuckGe
cargo run --release examples/hello.bf
```
A "HELLO" message should appear. 

# How-to

This code paints a whole 32x32 canvas white
```
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
- Fix resolution glitches - Can't have anything but a square. If y > x the white spots start multiplying and if x > y the program panics.  Try:

```
128£16£*£
+[>+*<->]
```
and
```
16£17£*£
+[>+*<->]
```
I know more or less what is causing this, but I'm too lazy to fix it.

# Done
- File to be used goes an argument, rather than always the main.bf
- Use '*' to update canvas
- Rewrite front-end (all in src/main.rs)
- Fixed main resolution glitch
