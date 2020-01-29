use rltk::{ VirtualKeyCode, Rltk, Point, console };
use specs::prelude::*;
use super::{ Position, Player, Map, TileType, State, Viewshed, RunState, CombatStats, WantsToMelee };
use std::cmp::{ min, max };


pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
  let mut positions = ecs.write_storage::<Position>();
  let mut players = ecs.write_storage::<Player>();
  let mut viewsheds = ecs.write_storage::<Viewshed>();
  let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();
  let mut entities = ecs.entities();
  let combat_stats = ecs.read_storage::<CombatStats>();
  let map = ecs.fetch::<Map>();

  for (entity, _player, pos, viewshed) in (&entities, &players, &mut positions, &mut viewsheds).join() {
    let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

    for potential_target in map.tile_content[destination_idx].iter() {
      let target = combat_stats.get(*potential_target);
      if let Some(_target) = target {
        wants_to_melee.insert(entity, WantsToMelee { target: *potential_target }).expect("Add target failed");
        return;
      }
     }

    let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
    if !map.blocked[destination_idx] {
      pos.x = min(79, max(0, pos.x + delta_x));
      pos.y = min(49, max(0, pos.y + delta_y));
      viewshed.dirty = true;

      let mut ppos = ecs.write_resource::<Point>();
      ppos.x = pos.x;
      ppos.y = pos.y;
    }
  }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
  match ctx.key {
    None => { return RunState::AwaitingInput }
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

      //Diagonals
      VirtualKeyCode::Numpad9 |
      VirtualKeyCode::Y => try_move_player(1, -1, &mut gs.ecs),
      VirtualKeyCode::Numpad7 |
      VirtualKeyCode::U => try_move_player(-1, -1, &mut gs.ecs),
      VirtualKeyCode::Numpad3 |
      VirtualKeyCode::N => try_move_player(1, 1, &mut gs.ecs),
      VirtualKeyCode::Numpad1 |
      VirtualKeyCode::B => try_move_player(-1, 1, &mut gs.ecs),

      _ => { return RunState::AwaitingInput }
    },
  }
  RunState::PlayerTurn
}