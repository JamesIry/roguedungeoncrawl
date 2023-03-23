use crate::prelude::*;

pub fn game_over_system(mut turn_state: ResMut<TurnState>, key_press: Res<KeyPress>) {
    let mut draw_batch = DrawBatch::new();

    draw_batch.target(2);
    draw_batch.print_color_centered(2, "Your quest has ended", ColorPair::new(RED, BLACK));
    draw_batch.print_color_centered(
        4,
        "Slain by a monster, your hero's journey has come to a premature end.",
        ColorPair::new(WHITE, BLACK),
    );
    draw_batch.print_color_centered(
        5,
        "The Amulet of Yala remains unclaimed, and your home town is not saved.",
        ColorPair::new(WHITE, BLACK),
    );
    draw_batch.print_color_centered(
        8,
        "Don't worry, you can always try again with a new hero.",
        ColorPair::new(YELLOW, BLACK),
    );
    draw_batch.print_color_centered(9, "Press 1 to play again.", ColorPair::new(GREEN, BLACK));

    draw_batch.submit(10000).expect("Batch error");

    if let Some(VirtualKeyCode::Key1) = key_press.0 {
        *turn_state = TurnState::InitGame;
    }
}
