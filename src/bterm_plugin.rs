use crate::prelude::*;

#[derive(Resource)]
pub struct BTermResource(pub BTerm);

struct BTermState {
    app: App,
}

impl GameState for BTermState {
    fn tick(&mut self, ctx: &mut BTerm) {
        for console in 0..=2 {
            ctx.set_active_console(console);
            ctx.cls();
        }

        self.app.insert_resource(KeyPress(ctx.key));
        ctx.set_active_console(0);
        self.app
            .insert_resource(Position(Point::from_tuple(ctx.mouse_pos())));

        // Dispatch systems
        self.app.update();

        // Render screen
        render_draw_buffer(ctx).expect("Couldn't render draw buffer");
    }
}

fn bterm_runner(mut app: App) {
    let context = app
        .world
        .remove_resource::<BTermResource>()
        .expect("BTerm context doesn't exist in the world, which is required in order to run");

    main_loop(context.0, BTermState { app }).expect("Could not start BTerm main loop");
}

pub struct BTermPlugin;

impl Plugin for BTermPlugin {
    fn build(&self, app: &mut App) {
        app.set_runner(bterm_runner);
    }
}
