use bracket_lib::prelude::*;


enum GameMode{
    Menu,
    Playing,
    End,
}

struct State{
    mode : GameMode
}

impl State {
    fn new() -> Self{

        return State {
                        mode : GameMode::Menu,
                     }
    }

    fn play(&mut self, ctx : &mut BTerm){
        self.mode = GameMode::End;
    }

    

    fn restart(&mut self, ctx : &mut BTerm){
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx : &mut BTerm){
        ctx.cls();
        ctx.print_centered(5,"Bienvenido al juego");
        ctx.print_centered(8,"(P) Play");
        ctx.print_centered(9,"(Q) Exit");


        if let Some(key) = ctx.key {

            match key {
                VirtualKeyCode::P => self.play(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }

        }

        
    }

    fn dead(&mut self, ctx : &mut BTerm){
        ctx.cls();
        ctx.print_centered(5,"Has perdido");
        ctx.print_centered(8,"(P) Play");
        ctx.print_centered(9,"(Q) Exit");

        if let Some(key) = ctx.key {

            match key {
                VirtualKeyCode::P => self.restart(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }

        }


    }

}




impl GameState for State {

    fn tick(&mut self, ctx: &mut BTerm){
        //ctx.cls();
        //ctx.print(1,1, "Cállise y mame ay !!");

        match self.mode{
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }

    }
   
}

fn main() -> BError{
    let context = BTermBuilder::simple80x50().with_title("Mi primer paso con un video juego tio !").build()?;
    main_loop(context, State::new())

}
