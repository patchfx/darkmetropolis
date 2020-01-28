use rltk::{ VirtualKeyCode, Rltk, Point };
use specs::prelude::*;
use super::{ Position, Player, Map, TileType, State, Viewshed, RunState };
use std::cmp::{ min, max };

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
  let mut positions = ecs.write_storage::<Position>();
  let mut players = ecs.write_storage::<Player>();
  let mut viewshed = ecs.write_storage::<Viewshed>();
  let map = ecs.fetch::<Map>();

  for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewshed).join() {
    let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
    if !map.blocked[destination_idx] {
      pos.x = min(79, max(0, pos.x + delta_x));
      pos.y = min(49, max(0, pos.y + delta_y));
      let mut ppos = ecs.write_resource::<Point>();
      ppos.x = pos.x;
      ppos.y = pos.y;
      viewshed.dirty = true;
    }
  }


}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
  match ctx.key {
    None => { return RunState::Paused }
    Some(key) => match key {
      VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
      VirtualKeyCode::Numpad4 => try_move_player(-1, 0, &mut gs.ecs),
      VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),
      VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
      VirtualKeyCode::Numpad6 => try_move_player(1, 0, &mut gs.ecs),
      VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),
      VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
      VirtualKeyCode::Numpad8 => try_move_player(0, -1, &mut gs.ecs),
      VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),
      VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
      VirtualKeyCode::Numpad2 => try_move_player(0, 1, &mut gs.ecs),
      VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),
      _ => { return RunState::Paused }
    },
  }
  RunState::Running
}