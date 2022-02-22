extern crate sdl2; //import sdl2 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::thread::sleep;



pub fn main(){
    let sdl_context=sdl2::init().expect("SDL initialization failed"); //initialize an SDL context
    let video_subsystem=sdl_context.video().expect("Couldn't get SDL video subsystem"); //Get the video subsystem



    //create window
    let window=video_subsystem.window("rust-sdl2 demo: video",800,600) 
                .position_centered()
                .opengl()
                .build()
                .expect("Failed to create window");

    let mut canvas=window.into_canvas().build().expect("Failed to convert window into canvas");


    canvas.set_draw_color(Color::RGB(255,0,0));
    canvas.clear();
    canvas.present();


    //Event handler
    let mut event_pump=sdl_context.event_pump().expect("Failed to get  SDL  event pump");


    //Event loop. NB: We need to add an event loop in order to keep the application running else it will quickly display a window and close it.
    
       'running:loop{

       for event in event_pump.poll_iter(){
            match event{
                Event::Quit{..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..}=>
                {
                    break 'running;
                },
                    _=>{}
                }
            }
            sleep(Duration::new(0,1_000_000_000u32/60));
        }
    }
