
pub struct BrainFuck {
    pub array: [u8; 1024],
}


pub fn brainfuck_compiler(program: &str, bf: &mut BrainFuck) {

    let program = program.as_bytes();

    let mut main_loop_index = 0;
    let mut array_index = 0;

    //It's a vector to allow nested loops     // The index where the loop started
    let mut in_loop: Vec<usize> = vec![0];


    while main_loop_index < program.len() {
        
        match program[main_loop_index] {
            
            b'>' => if array_index < 1023 {array_index += 1},
            
            b'<' => if array_index > 0 {array_index -= 1},
            
            b'+' => if bf.array[array_index] < 255 {bf.array[array_index] += 1},
            
            b'-' => if bf.array[array_index] > 0 {bf.array[array_index] -= 1},
            
            b'.' => print!("{}", bf.array[array_index] as char),
            
            b'[' => in_loop.push(main_loop_index),
            
            b']' => if bf.array[array_index] == 0 {in_loop.pop();} else {main_loop_index = *in_loop.last().unwrap_or(&main_loop_index)},
            
            b',' => {
                let key = program[main_loop_index + 1];



                
                //      let mut input: [u8; 1] = [0; 1];
                //      std::io::stdin().read_exact(&mut input).expect("Failed to read stdin");
                //      bf.array[array_index] = input[0]; 
                // These 3 lines were taken from  https://github.com/Overv/bf/blob/master/src/main.rs with only slight changes
                // They represent how it *should* be programmed. Unfortunately, it doesn't work for me.
            },
    
            _ => {}
        };

        main_loop_index += 1;
    }
}