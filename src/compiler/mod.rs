use ruscii::{app::State, keyboard::{KeyEvent, Key}};


pub struct BrainFuck {
    pub array: Vec<u8>,
    pub main_loop_index: usize,
    pub array_index: usize,

    //It's a vector to allow nested loops     // The index where the loop started
    pub in_loop:                                Vec<usize>,
}


pub fn brainfuck_compiler(program: &str, bf: &mut BrainFuck, app_state: &mut State, (prefx, prefy): (usize, usize)) {

    let program = program.as_bytes();

    // The following line would be what a normal brainfuck compiler would have
    // while main_loop_index < program.len() {
    loop {
        if bf.main_loop_index >= program.len() {
            bf.main_loop_index = 0;
        }

        match program[bf.main_loop_index] {
            
            b'>' => if bf.array_index < prefx * prefy - 1 {bf.array_index += 1},
            
            b'<' => if bf.array_index > 0 {bf.array_index -= 1},
            
            b'+' => if bf.array[bf.array_index] < 255 {bf.array[bf.array_index] += 1},
            
            b'-' => if bf.array[bf.array_index] > 0 {bf.array[bf.array_index] -= 1},
            
            b'.' => print!("{}", bf.array[bf.array_index] as char),
            
            b'[' => bf.in_loop.push(bf.main_loop_index),
            
            b']' => if bf.array[bf.array_index] == 0 {bf.in_loop.pop();} else {bf.main_loop_index = *bf.in_loop.last().unwrap_or(&bf.main_loop_index)},
            
            b',' => {
                let key = program[bf.main_loop_index + 1];

                for key_event in app_state.keyboard().last_key_events() {
                    match key_event {
                        KeyEvent::Pressed(Key::W) => if key == b'w' {bf.array[bf.array_index] = b'w';},
                        KeyEvent::Pressed(Key::A) => if key == b'a' {bf.array[bf.array_index] = b'a';},
                        KeyEvent::Pressed(Key::S) => if key == b's' {bf.array[bf.array_index] = b's';},
                        KeyEvent::Pressed(Key::D) => if key == b'd' {bf.array[bf.array_index] = b'd';},
                        _ => (),
                    }
                }
                
                // let mut input: [u8; 1] = [0; 1];
                // std::io::stdin().read_exact(&mut input).expect("Failed to read stdin");
                // bf.array[array_index] = input[0]; 
                // These 3 lines were taken from  https://github.com/Overv/bf/blob/master/src/main.rs with only slight changes
                // They represent how it *should* be programmed. Unfortunately, it doesn't work for me.
            },

            b'*' => {bf.main_loop_index += 1;break;},
    
            _ => {}
        };

        bf.main_loop_index += 1;
    }
}