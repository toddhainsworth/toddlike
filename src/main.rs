use bracket_lib::prelude::*;
use specs::prelude::*;
use specs::World;

mod entities;

struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, context: &mut BTerm) {
        let positions = self.ecs.read_storage::<entities::Position>();
        let renderables = self.ecs.read_storage::<entities::Renderable>();

        for (position, renderable) in (&positions, &renderables).join() {
            context.set(
                position.x,
                position.y,
                renderable.fg_col,
                renderable.bg_col,
                renderable.glyph,
            );
        }
    }
}

fn main() -> BError {
    let mut state = State { ecs: World::new() };
    let context = BTermBuilder::simple80x50()
        .with_title("Toddlilke")
        .build()?;

    state.ecs.register::<entities::Position>();
    state.ecs.register::<entities::Renderable>();

    state
        .ecs
        .create_entity()
        .with(entities::Position::new(10, 10))
        .with(entities::Renderable::new('@', YELLOW, BLACK))
        .build();
    main_loop(context, state)
}
