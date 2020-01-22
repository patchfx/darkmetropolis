extern crate rltk;

use rltk::{Console, GameState, Rltk, RGB };
use specs::prelude::*;

#[macro_use]
extern crate specs_derive;

mod map;
pub use map::*;

mod player;
pub use player::*;

mod component;
pub use component::*;

mod rect;
pub use rect::*;

mod visibility_system;
pub use visibility_system::VisibilitySystem;

pub struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        player_input(self, ctx);
        self.run_systems();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        draw_map(&self.ecs, ctx);

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() {
    let context = Rltk::init_simple8x8(80, 50, "Dark Metropolis", "resources");
    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    gs.ecs.insert(map);
    gs.ecs
      .create_entity()
      .with(Position { x: player_x, y: player_y })
      .with(Renderable {
          glyph: rltk::to_cp437('@'),
          fg: RGB::named(rltk::YELLOW),
          bg: RGB::named(rltk::BLACK),
      })
      .with(Player{})
      .with(Viewshed { visible_tiles: Vec::new(), range: 8, dirty: true })
      .build();
    rltk::main_loop(context, gs);
}
