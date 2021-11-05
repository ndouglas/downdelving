use crate::map_builders::prefab_builder::prefab_sections::*;
use crate::map_builders::{
    AreaEndingPosition, AreaStartingPosition, BuilderChain, CellularAutomataBuilder,
    CullUnreachable, PrefabBuilder, VoronoiSpawning, WaveformCollapseBuilder, XEnd, XStart, YEnd,
    YStart,
};

pub fn mushroom_exit(new_depth: i32, width: i32, height: i32) -> BuilderChain {
    let mut chain = BuilderChain::new(new_depth, width, height, "Into The Mushroom Grove");
    chain.start_with(CellularAutomataBuilder::new());
    chain.with(WaveformCollapseBuilder::new());
    chain.with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER));
    chain.with(CullUnreachable::new());
    chain.with(AreaStartingPosition::new(XStart::RIGHT, YStart::CENTER));
    chain.with(AreaEndingPosition::new(XEnd::LEFT, YEnd::CENTER));
    chain.with(VoronoiSpawning::new());
    chain.with(PrefabBuilder::sectional(DROW_ENTRY));
    chain
}
