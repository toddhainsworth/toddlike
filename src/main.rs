use bracket_lib::prelude::*;
use specs::prelude::*;
use specs::World;

mod entities;
mod io;

struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, context: &mut BTerm) {
        context.cls();

        handle_player_input(self, context);

        let positions = self.ecs.read_storage::<entities::Position>();
        let renderables = self.ecs.read_storage::<entities::Renderable>();
        let players = self.ecs.read_storage::<entities::Player>();

        for (position, renderable) in (&positions, &renderables).join() {
            context.set(
                position.x,
                position.y,
                renderable.fg_col,
                renderable.bg_col,
                renderable.glyph,
            );
        }
        for (_player, position) in (&players, &positions).join() {
            context.print(1, 48, format!("X: {} Y: {}", position.x, position.y));
        }
    }
}

fn handle_player_input(state: &mut State, context: &mut BTerm) {
    if let Some(key) = context.key {
        match key {
            VirtualKeyCode::Left => move_player(-1, 0, &mut state.ecs, &context),
            VirtualKeyCode::Right => move_player(1, 0, &mut state.ecs, &context),
            VirtualKeyCode::Up => move_player(0, -1, &mut state.ecs, &context),
            VirtualKeyCode::Down => move_player(0, 1, &mut state.ecs, &context),
            VirtualKeyCode::Q => context.quit(),
            _ => {}
        }
    }
}

fn xy_index(x: i32, y: i32, context: &BTerm) -> usize {
    (y as usize * context.width_pixels as usize) + x as usize
}

fn move_player(delta_x: i32, delta_y: i32, ecs: &mut World, context: &BTerm) {
    let mut positions = ecs.write_storage::<entities::Position>();
    let mut players = ecs.write_storage::<entities::Player>();

    for (_player, position) in (&mut players, &mut positions).join() {
        let x = position.x + delta_x;
        let y = position.y + delta_y;
        let _destination = xy_index(x, y, context);

        // FIXME: Don't cast, could cut off
        if (x <= context.width_pixels as i32) && x > 0 {
            position.x = position.x + delta_x;
        }
        if (y <= context.height_pixels as i32) && y > 0 {
            position.y = position.y + delta_y;
        }
    }
}

fn main() -> BError {
    let mut state = State { ecs: World::new() };
    let context = BTermBuilder::simple80x50().with_title("Toddlike").build()?;

    state.ecs.register::<entities::Position>();
    state.ecs.register::<entities::Renderable>();
    state.ecs.register::<entities::Player>();

    state
        .ecs
        .create_entity()
        .with(entities::Position::new(10, 10))
        .with(entities::Renderable::new('@', YELLOW, BLACK))
        .with(entities::Player::new())
        .build();
    main_loop(context, state)
}
