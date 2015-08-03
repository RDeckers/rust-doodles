extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston_window;

use std::io;
use std::io::prelude::*;
use piston_window::*;
use opengl_graphics::{ GlGraphics };

pub struct Display{
  gl: GlGraphics,
  should_redraw: bool,
  clear_colour: [f32; 4],
  cursor_position:  (f64, f64)
}

impl Display{
  pub fn new(gl_version: OpenGL) -> Self{
    Display{
      should_redraw: true,
      gl: GlGraphics::new(gl_version),
      clear_colour: [1f32;4],
      cursor_position: (0.0, 0.0)
    }
  }

  pub fn clear(&mut self, args: RenderArgs){
  }

  pub fn draw_cursor(&mut self, args: RenderArgs, position: (u32, u32)){
    const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    let square = rectangle::square(0.0, 0.0, 50.0);
    self.clear(args);
    self.gl.draw(args.viewport(), |_c, g| {
      rectangle(RED, square, _c.transform, g);
    });
  }

  pub fn draw(&mut self, args: RenderArgs){
    const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    let square = rectangle::square(self.cursor_position.0, self.cursor_position.1, 50.0);
    let borrowed_colour = self.clear_colour;
    self.gl.draw(args.viewport(), |_c, g| {
      clear(borrowed_colour, g);
      rectangle(RED, square, _c.transform, g);
    });
    self.should_redraw = false;
  }

  pub fn move_cursor(&mut self, x: f64, y: f64){
    self.cursor_position = (x,y);
    self.should_redraw = true;
  }

  pub fn update(&mut self, args:UpdateArgs){
    //self.should_redraw = true;
  }
}

fn main() {
  let opengl = OpenGL::V3_2; //Why V?! http://www.rust-ci.org/PistonDevelopers/piston/doc/shader_version/opengl/enum.OpenGL.html
  {//actual main loop, makes sure the window gets dropped (i.e. dissapears).
    let mut window: PistonWindow = WindowSettings::new("title", [800,640]).fullscreen(false).opengl(opengl).into(); //can't copy?
    let mut display = Display::new(opengl);
    for e in window.clone().events().ups(10_000).max_fps(120){//TODO: How does this work, keypresses and updates are seperate?
      //poll event instead?
      match e{
        Event::Render(args) /*if display.should_redraw*/ => {
          println!("Redrawing");
          display.draw(args);
        }
        Event::Update(args) => {
          //display.update(args);
        }
        Event::Input(Input::Move(Motion::MouseCursor(x, y))) => {
          //println!("Mouse move @ {} {}", x, y);
          display.move_cursor(x,y);
        }
        Event::Input(Input::Press(Button::Keyboard(key))) => {
          println!("Keypressed {:?}!", key);//???
          if key == Key::Space {
            window.set_should_close(true);
          }
        }
        _ => {}
      }
    }
  }
}



    /*

    initialize shit
    Fetch events.
    Pass events to handling thread. (Put events in FIFO queue?)
    Can't acces glContext directly, can change cpu-side state and flag for bufferswap.
    swap buffers (if neccesary).

    */

/*
  -Mouse tracking lags behind "real" mouse?
*/
