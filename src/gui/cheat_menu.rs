use super::{menu_box, menu_option};
use crate::State;
use rltk::prelude::*;
use rltk::to_cp437;

#[derive(PartialEq, Copy, Clone)]
pub enum CheatMenuResult {
    NoResponse,
    Cancel,
    TeleportToExit,
    Heal,
    Reveal,
    GodMode,
    LevelUp,
    Eat,
}

pub fn show_cheat_mode(_gs: &mut State, ctx: &mut Rltk) -> CheatMenuResult {
    let mut draw_batch = DrawBatch::new();
    let count = 6;
    let mut y = (25 - (count / 2)) as i32;
    menu_box(&mut draw_batch, 15, y, 31, (count + 3) as i32, "Cheating!");
    draw_batch.print_color(
        Point::new(18, y + count as i32 + 1),
        "[Esc] cancel",
        ColorPair::new(RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK)),
    );

    menu_option(
        &mut draw_batch,
        17,
        y,
        to_cp437('T'),
        "Teleport to next level",
    );
    y += 1;
    menu_option(&mut draw_batch, 17, y, to_cp437('H'), "Heal all wounds");
    y += 1;
    menu_option(&mut draw_batch, 17, y, to_cp437('R'), "Reveal the map");
    y += 1;
    menu_option(&mut draw_batch, 17, y, to_cp437('G'), "God Mode (No Death)");
    y += 1;
    menu_option(&mut draw_batch, 17, y, to_cp437('L'), "Level up");
    y += 1;
    menu_option(&mut draw_batch, 17, y, to_cp437('E'), "Eat");

    draw_batch
        .submit(6000)
        .map_err(|err| println!("{:?}", err))
        .ok();

    match ctx.key {
        None => CheatMenuResult::NoResponse,
        Some(key) => match key {
            VirtualKeyCode::T => CheatMenuResult::TeleportToExit,
            VirtualKeyCode::H => CheatMenuResult::Heal,
            VirtualKeyCode::R => CheatMenuResult::Reveal,
            VirtualKeyCode::G => CheatMenuResult::GodMode,
            VirtualKeyCode::L => CheatMenuResult::LevelUp,
            VirtualKeyCode::E => CheatMenuResult::Eat,
            VirtualKeyCode::Escape => CheatMenuResult::Cancel,
            _ => CheatMenuResult::NoResponse,
        },
    }
}
