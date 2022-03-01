extern crate sdl2; //import sdl2 

 
use std::time::SystemTime;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas,Texture, TextureCreator};
use std::thread::sleep;
use std::time::Duration;
use sdl2::video::{Window,WindowContext};




#[derive(Clone,Copy)]
enum TextureColor{
    Green,
    Blue
}




fn create_texture_rect<'a>(canvas: &mut Canvas<Window>,texture_creator:&'a TextureCreator<WindowContext>, color:TextureColor ,size:u32)->Option<Texture<'a>>{
// We will want to handle failures outside of this function.WindowContext

if let Ok(mut square_texture)=
    texture_creator.create_texture_target(None, size, size){
        canvas.with_texture_canvas(&mut square_texture,|texture| {
            match color{
                TextureColor::Green=>
                texture.set_draw_color(Color::RGB(0, 255, 0)),
                TextureColor::Blue=>
                texture.set_draw_color(Color::RGB(0, 0, 255)),
            }
            texture.clear();
        }).expect("Failed to color a texture");
        Some(square_texture)
    }else{
        None
    }
}  






pub fn main(){
    let sdl_context=sdl2::init().expect("SDL initialization failed"); //initialize an SDL context
    let video_subsystem=sdl_context.video().expect("Couldn't get SDL video subsystem"); //Get the video subsystem



    //create window. Parameters are: title, width, height
    let window=video_subsystem.window("Tetris",800,600) 
                .position_centered()
                .build()
                .expect("Failed to create window");

 let mut  canvas = window.into_canvas() //    transforms the window into a canvas so that we can manipulate it more easiluy
                    .target_texture() // activates texture rendering support
                    .present_vsync()  // enables the v-sync (also known as vertical-synchronization) limit
                    .build() // creates the canvas by applying all previously set parameters
                    .expect("Couldn't get window's canvas");
    
                    // let mut canvas = window.into_canvas()
                    // .target_texture()
                    // .present_vsync() // To enable v-sync.
                    // .build()
                    // .expect("Couldn't get window's canvas");
                

                    let texture_creator: TextureCreator<_> = canvas.texture_creator();

                    const TEXTURE_SIZE: u32=32;

    //create a texture with a 323X32 size
    
    let green_square =create_texture_rect(&mut canvas, &texture_creator, TextureColor::Green, TEXTURE_SIZE).expect("Failed to create a texture");

    let blue_squre=create_texture_rect(&mut canvas, &texture_creator, TextureColor::Blue, TEXTURE_SIZE).expect("Failed to creadte blue texture");

    let timer=SystemTime::now();

    //First we get the event handler

    //create a square texture with a size 32X32
//    let mut square_texture:Texture=texture_creator.create_texture_target(None,TEXTURE_SIZE, TEXTURE_SIZE)
//                             .expect("Failed to create a texture");


//         //use the canvas to draw into square matrix
//         canvas.with_texture_canvas(&mut square_texture, |texture|{
//             texture.set_draw_color(Color::RGB(0,255,0));  //set color to be used when drawing occurs
//             texture.clear(); // clears the texture so it'll be filled with the color set with set_draw_color
//     })
//         .expect("Failed to color a texture");



    //First we get the event handler;
    let mut event_pump=sdl_context.event_pump().expect("Failed to get  SDL  event pump");


    //Event loop. NB: We need to add an event loop in order to keep the application running else it will quickly display a window and close it.
    
       'running:loop{

       for event in event_pump.poll_iter(){
            match event{
                // terminates window when close button is clicked on window
                Event::Quit{..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..}=>
                { 
                    break 'running;
                },
                    _=>{}
                }
            }
            //fill the window with red
            canvas.set_draw_color(Color::RGB(255,0,0));

            //draw it
            canvas.clear();
            
             // the rectangle switch happens here
             let display_green=match timer.elapsed(){
                 Ok(elapsed)=>elapsed.as_secs()%2==0,
                 Err(_)=>{
                     // in case of error, we do nothing...
                     true
                 }
             };

             let square_texture=if display_green{
                 &green_square
                }else{
                    &blue_squre
                };

                //copy texture into the window
                canvas.copy(square_texture,None,
                //we copy it at the top-left of the window with a 32X32 size
                Rect::new(0,0,TEXTURE_SIZE,TEXTURE_SIZE)).expect("Couldn't copy texture into window");
                // update Window's 
                canvas.present();

                //we sleep enough to get ~60 fps. if we don't call this, the program will take 100%  of a CPU time
                sleep(Duration::new(0,1_000_000_000u32/60));


                

            
            //update the window's display
            canvas.present();
            sleep(Duration::new(0,1_000_000_000u32/60));
        }



    }
