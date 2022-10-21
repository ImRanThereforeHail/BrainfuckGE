use std::fs;
use std::env;
use ruscii::app::{App, State};
use ruscii::drawing::RectCharset;
use ruscii::terminal::{Window};
use ruscii::drawing::{Pencil};
use ruscii::keyboard::{KeyEvent, Key};
use ruscii::spatial::{Vec2};
use ruscii::gui::{FPSCounter};

mod compiler;

fn main() {
    if env::args().len() < 2 { panic!("Please provide a brainfuck source file") }
    
    let mut logic = String::new();
    for argument in env::args() {
        // Chack if this is the first iteneration of the loop
        if logic.len() == 0 {
            logic += " ";
            continue;
        }
        // Check if source code has been written
        if logic.len() > 1 { panic!("More than 1 source file was given!")}

        logic += &fs::read_to_string(&argument).expect("Path does not exist");
    }

    // Delete unnecessary stuff
    //let logic = logic.chars().filter(|i| "><-+.[],*".contains(*i)).fold(String::new(), |init, char| init + &char.to_string());
    // Cannot do this due to the way the input works
    
    // Get the headers
    let headers = &logic.split('£').collect::<Vec<&str>>();
    let (prefx, prefy): (usize, usize) = (headers[0].trim().parse().unwrap(), headers[1].trim().parse().unwrap());


    let mut bf = compiler::BrainFuck {
        array: vec![vec![0; (prefx*prefy).try_into().unwrap()], vec![0; (prefx*prefy).try_into().unwrap()]],
        main_loop_index: 0,
        array_index: 0,
        in_loop: Vec::new(),
        current_array: 0,
    };

    let mut fps_counter = FPSCounter::new();
    let mut app = App::new();

    let mut is_first_iteration = true;
    app.run(|app_state: &mut State, window: &mut Window| {
        if is_first_iteration {
            compiler::brainfuck_compiler(headers[2], &mut bf, app_state, (prefx, prefy));
            is_first_iteration = false;
        }

        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        compiler::brainfuck_compiler(&headers[3], &mut bf, app_state, (prefx, prefy));

        let (wx,wy) = (window.size().x, window.size().y);

        let mut pencil = Pencil::new(window.canvas_mut());
        pencil.draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(0, prefy));

        for y in 0..prefy {
            for x in 0..prefx {
                let is_painted = bf.array[0][y * prefy + x] > 0;
                if !is_painted { continue; }
                
                pencil.set_background(ruscii::terminal::Color::White);
                pencil.draw_char(' ', Vec2::xy(x, y));
                /* pencil
                .draw_rect(
                    &RectCharset::simple_lines(),
                    Vec2::xy((wx / prefx as i32) * x as i32, (wy / prefy as i32) * y as i32),
                    Vec2::xy(wx / prefx as i32,wy / prefy as i32)
                ); */
            }
        }


        fps_counter.update();
    });
}
