use super::{get_item_color, get_item_display_name, menu_box};
use crate::vendor::{VendorMode, VendorResult};
use crate::{InBackpack, Item, Name, State, Vendor};
use rltk::prelude::*;
use specs::prelude::*;

const VENDOR_MENU_WIDTH: i32 = 45;

fn vendor_sell_menu(
    gs: &mut State,
    ctx: &mut Rltk,
    _vendor: Entity,
    _mode: VendorMode,
) -> (VendorResult, Option<Entity>, Option<String>, Option<f32>) {
    let mut draw_batch = DrawBatch::new();
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<InBackpack>();
    let items = gs.ecs.read_storage::<Item>();
    let entities = gs.ecs.entities();
    let black = RGB::named(rltk::BLACK);
    let yellow_on_black = ColorPair::new(RGB::named(rltk::YELLOW), black);
    let white_on_black = ColorPair::new(RGB::named(rltk::WHITE), black);

    let inventory = (&backpack, &names)
        .join()
        .filter(|item| item.0.owner == *player_entity);
    let count = inventory.count();

    let mut y = (25 - (count / 2)) as i32;

    menu_box(
        &mut draw_batch,
        15,
        y,
        VENDOR_MENU_WIDTH,
        (count + 3) as i32,
        "Sell Which Item?",
    );
    draw_batch.print_color(
        Point::new(18, y + count as i32 + 1),
        "[Esc] cancel [Space] buy",
        yellow_on_black,
    );

    let mut equippable: Vec<Entity> = Vec::new();
    let mut j = 0;
    for (entity, _pack, item) in (&entities, &backpack, &items)
        .join()
        .filter(|item| item.1.owner == *player_entity)
    {
        draw_batch.set(Point::new(17, y), white_on_black, rltk::to_cp437('('));
        draw_batch.set(
            Point::new(18, y),
            yellow_on_black,
            97 + j as rltk::FontCharType,
        );
        draw_batch.set(Point::new(19, y), white_on_black, rltk::to_cp437(')'));
        draw_batch.print_color(
            Point::new(21, y),
            &get_item_display_name(&gs.ecs, entity),
            ColorPair::new(get_item_color(&gs.ecs, entity), black),
        );
        draw_batch.print(
            Point::new(50, y),
            &format!("{:.1} gp", item.base_value * 0.8),
        );
        equippable.push(entity);
        y += 1;
        j += 1;
    }

    draw_batch
        .submit(6000)
        .map_err(|err| println!("{:?}", err))
        .ok();

    match ctx.key {
        None => (VendorResult::NoResponse, None, None, None),
        Some(key) => match key {
            VirtualKeyCode::Space => (VendorResult::BuyMode, None, None, None),
            VirtualKeyCode::Escape => (VendorResult::Cancel, None, None, None),
            _ => {
                let selection = rltk::letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    return (
                        VendorResult::Sell,
                        Some(equippable[selection as usize]),
                        None,
                        None,
                    );
                }
                (VendorResult::NoResponse, None, None, None)
            }
        },
    }
}

fn vendor_buy_menu(
    gs: &mut State,
    ctx: &mut Rltk,
    vendor: Entity,
    _mode: VendorMode,
) -> (VendorResult, Option<Entity>, Option<String>, Option<f32>) {
    use crate::raws::*;
    let mut draw_batch = DrawBatch::new();

    let vendors = gs.ecs.read_storage::<Vendor>();

    let inventory = crate::raws::get_vendor_items(
        &vendors.get(vendor).unwrap().categories,
        &RAWS.lock().unwrap(),
    );
    let count = inventory.len();
    let black = RGB::named(rltk::BLACK);
    let yellow_on_black = ColorPair::new(RGB::named(rltk::YELLOW), black);
    let white_on_black = ColorPair::new(RGB::named(rltk::WHITE), black);

    let mut y = (25 - (count / 2)) as i32;
    menu_box(
        &mut draw_batch,
        15,
        y,
        VENDOR_MENU_WIDTH,
        (count + 3) as i32,
        "Buy Which Item?",
    );
    draw_batch.print_color(
        Point::new(18, y + count as i32 + 1),
        "[Esc] cancel [Space] sell",
        yellow_on_black,
    );

    for (j, sale) in inventory.iter().enumerate() {
        draw_batch.set(Point::new(17, y), white_on_black, rltk::to_cp437('('));
        draw_batch.set(
            Point::new(18, y),
            yellow_on_black,
            97 + j as rltk::FontCharType,
        );
        draw_batch.set(Point::new(19, y), white_on_black, rltk::to_cp437(')'));
        draw_batch.print(Point::new(21, y), &sale.0);
        draw_batch.print(Point::new(50, y), &format!("{:.1} gp", sale.1 * 1.2));
        y += 1;
    }

    draw_batch
        .submit(6000)
        .map_err(|err| println!("{:?}", err))
        .ok();

    match ctx.key {
        None => (VendorResult::NoResponse, None, None, None),
        Some(key) => match key {
            VirtualKeyCode::Space => (VendorResult::SellMode, None, None, None),
            VirtualKeyCode::Escape => (VendorResult::Cancel, None, None, None),
            _ => {
                let selection = rltk::letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    return (
                        VendorResult::Buy,
                        None,
                        Some(inventory[selection as usize].0.clone()),
                        Some(inventory[selection as usize].1),
                    );
                }
                (VendorResult::NoResponse, None, None, None)
            }
        },
    }
}

pub fn show_vendor_menu(
    gs: &mut State,
    ctx: &mut Rltk,
    vendor: Entity,
    mode: VendorMode,
) -> (VendorResult, Option<Entity>, Option<String>, Option<f32>) {
    match mode {
        VendorMode::Buy => vendor_buy_menu(gs, ctx, vendor, mode),
        VendorMode::Sell => vendor_sell_menu(gs, ctx, vendor, mode),
    }
}
