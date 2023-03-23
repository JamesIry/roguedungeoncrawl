use crate::prelude::*;

pub fn victory_system(mut turn_state: ResMut<TurnState>, key_press: Res<KeyPress>) {
    let mut draw_batch = DrawBatch::new();

    draw_batch.target(2);
    draw_batch.print_color_centered(2, "You have won!", ColorPair::new(GREEN, BLACK));
    draw_batch.print_color_centered(
        4,
        "You put on the Amulet of Yala and feel its power course through your veins.",
        ColorPair::new(WHITE, BLACK),
    );
    draw_batch.print_color_centered(
        5,
        "Your town is saved and you can return to your normal life.",
        ColorPair::new(WHITE, BLACK),
    );
    draw_batch.print_color_centered(7, "Press 1 to play again.", ColorPair::new(GREEN, BLACK));

    draw_batch.submit(10000).expect("Batch error");

    if let Some(VirtualKeyCode::Key1) = key_press.0 {
        *turn_state = TurnState::InitGame;
    }
}
