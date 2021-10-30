use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};

mod components;
pub use components::*;

pub fn register_all(ecs: &mut World) {
    ecs.register::<Position>();
    ecs.register::<Renderable>();
    ecs.register::<Player>();
    ecs.register::<Viewshed>();
    ecs.register::<Name>();
    ecs.register::<BlocksTile>();
    ecs.register::<WantsToMelee>();
    ecs.register::<Item>();
    ecs.register::<ProvidesHealing>();
    ecs.register::<InflictsDamage>();
    ecs.register::<AreaOfEffect>();
    ecs.register::<Consumable>();
    ecs.register::<Ranged>();
    ecs.register::<InBackpack>();
    ecs.register::<WantsToPickupItem>();
    ecs.register::<WantsToUseItem>();
    ecs.register::<WantsToDropItem>();
    ecs.register::<Confusion>();
    ecs.register::<SimpleMarker<SerializeMe>>();
    ecs.register::<SerializationHelper>();
    ecs.register::<DMSerializationHelper>();
    ecs.register::<Equippable>();
    ecs.register::<Equipped>();
    ecs.register::<Weapon>();
    ecs.register::<Wearable>();
    ecs.register::<WantsToRemoveItem>();
    ecs.register::<ParticleLifetime>();
    ecs.register::<HungerClock>();
    ecs.register::<ProvidesFood>();
    ecs.register::<MagicMapper>();
    ecs.register::<Hidden>();
    ecs.register::<EntryTrigger>();
    ecs.register::<EntityMoved>();
    ecs.register::<SingleActivation>();
    ecs.register::<BlocksVisibility>();
    ecs.register::<Door>();
    ecs.register::<Quips>();
    ecs.register::<Attributes>();
    ecs.register::<Skills>();
    ecs.register::<Pools>();
    ecs.register::<NaturalAttackDefense>();
    ecs.register::<LootTable>();
    ecs.register::<OtherLevelPosition>();
    ecs.register::<LightSource>();
    ecs.register::<Initiative>();
    ecs.register::<MyTurn>();
    ecs.register::<Faction>();
    ecs.register::<WantsToApproach>();
    ecs.register::<WantsToFlee>();
    ecs.register::<MoveMode>();
    ecs.register::<Chasing>();
    ecs.register::<EquipmentChanged>();
    ecs.register::<Vendor>();
    ecs.register::<TownPortal>();
    ecs.register::<TeleportTo>();
    ecs.register::<ApplyMove>();
    ecs.register::<ApplyTeleport>();
    ecs.register::<MagicItem>();
    ecs.register::<ObfuscatedName>();
    ecs.register::<IdentifiedItem>();
    ecs.register::<SpawnParticleBurst>();
    ecs.register::<SpawnParticleLine>();
    ecs.register::<CursedItem>();
    ecs.register::<ProvidesRemoveCurse>();
    ecs.register::<ProvidesIdentification>();
    ecs.register::<AttributeBonus>();
    ecs.register::<Duration>();
    ecs.register::<StatusEffect>();
    ecs.register::<KnownSpells>();
    ecs.register::<SpellTemplate>();
    ecs.register::<WantsToCastSpell>();
    ecs.register::<TeachesSpell>();
    ecs.register::<ProvidesMana>();
    ecs.register::<Slow>();
    ecs.register::<DamageOverTime>();
    ecs.register::<SpecialAbilities>();
    ecs.register::<TileSize>();
    ecs.register::<OnDeath>();
    ecs.register::<AlwaysTargetsSelf>();
    ecs.register::<Target>();
    ecs.register::<WantsToShoot>();
    ecs.register::<Bleeds>();
    ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());
}
