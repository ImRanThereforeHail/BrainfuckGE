use std::fs;
use pixel_canvas::{Canvas, Color, Image};

mod compiler;

fn main() {
    let mut bf = compiler::BrainFuck {
        array: [0; 1024],
    };
    let f = fs::read_to_string("main.bf").unwrap();

    let canvas = Canvas::new(32, 32)
                                .title("It'sa me, mario!");

    
    canvas.render(
        move |_, image: &mut Image| {
            compiler::brainfuck_compiler(&f, &mut bf);
            
            let white = Color { r: 255, g: 255, b: 255 };
            let black = Color { r: 0, g: 0, b: 0 };

            for (y, row) in image.chunks_mut(32).enumerate() {
                for (x, pixel) in row.iter_mut().enumerate() {
                    let value = bf.array[y * 32 + x];
                    if value > 0 {
                        *pixel = white;
                    } else {
                        *pixel = black;
                    }
                }
            }
            
        }
    );
}
