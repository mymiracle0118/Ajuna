use super::*;

pub(crate) const MIN_SACRIFICE: usize = 1;
pub(crate) const MAX_SACRIFICE: usize = 4;

pub(crate) const MAX_QUANTITY: u8 = 16;

pub(crate) const SCALING_FACTOR_PERC: u32 = 100;
pub(crate) const STACK_PROB_PERC: u32 = 10;
pub(crate) const TOOLBOX_PERC: u32 = 15;
pub(crate) const PROGRESS_PROBABILITY_PERC: u32 = 15;
pub(crate) const PROGRESS_VARIATIONS: u8 = 6;
pub(crate) const BASE_PROGRESS_PROB_PERC: u32 = 20;
pub(crate) const MAX_EQUIPPED_SLOTS: usize = 5;
pub(crate) const MAX_TOOLBOXES: usize = 2;
pub(crate) const GLIMMER_PROB_PERC: u32 = 15;
pub(crate) const COLOR_GLOW_SPARK: u32 = 52;
pub(crate) const SPARK_PROGRESS_PROB_PERC: u32 = 25;
pub(crate) const MAX_FLASK_PROGRESS: usize = 4;
pub(crate) const MATCH_ALGO_START_RARITY: RarityTier = RarityTier::Epic;

pub(crate) const MAX_BYTE: u32 = u8::MAX as u32;

pub(crate) const GLIMMER_FORGE_GLIMMER_USE: u8 = 1;
pub(crate) const GLIMMER_FORGE_MATERIAL_USE: u8 = 4;
pub(crate) const GLIMMER_FORGE_TOOLBOX_USE: u8 = 100;

pub(crate) const GLIMMER_SP: u8 = 2;

pub(crate) const PET_MOON_PHASE_SIZE: u32 = 10;
pub(crate) const PET_MOON_PHASE_AMOUNT: u32 = 14;

/// Probabilities for all PackType::Material options
pub(crate) const PACK_TYPE_MATERIAL_ITEM_PROBABILITIES: ProbabilitySlots<ItemType, 6> = [
	(ItemType::Pet, 140),
	(ItemType::Material, 750),
	(ItemType::Essence, 50),
	(ItemType::Equippable, 10),
	(ItemType::Blueprint, 0),
	(ItemType::Special, 50),
];

pub(crate) const PACK_TYPE_MATERIAL_PET_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<PetItemType, 3> =
	[(PetItemType::Pet, 0), (PetItemType::PetPart, 980), (PetItemType::Egg, 20)];

pub(crate) const PACK_TYPE_MATERIAL_MATERIAL_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	MaterialItemType,
	8,
> = [
	(MaterialItemType::Polymers, 135),
	(MaterialItemType::Electronics, 145),
	(MaterialItemType::PowerCells, 135),
	(MaterialItemType::Optics, 105),
	(MaterialItemType::Metals, 115),
	(MaterialItemType::Ceramics, 145),
	(MaterialItemType::Superconductors, 115),
	(MaterialItemType::Nanomaterials, 105),
];

pub(crate) const PACK_TYPE_MATERIAL_ESSENCE_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	EssenceItemType,
	5,
> = [
	(EssenceItemType::Glimmer, 940),
	(EssenceItemType::ColorSpark, 25),
	(EssenceItemType::GlowSpark, 25),
	(EssenceItemType::PaintFlask, 5),
	(EssenceItemType::GlowFlask, 5),
];

pub(crate) const PACK_TYPE_MATERIAL_EQUIPABLE_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	EquippableItemType,
	7,
> = [
	(EquippableItemType::ArmorBase, 970),
	(EquippableItemType::ArmorComponent1, 0),
	(EquippableItemType::ArmorComponent2, 0),
	(EquippableItemType::ArmorComponent3, 0),
	(EquippableItemType::WeaponVersion1, 15),
	(EquippableItemType::WeaponVersion2, 10),
	(EquippableItemType::WeaponVersion3, 5),
];

pub(crate) const PACK_TYPE_MATERIAL_BLUEPRINT_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	BlueprintItemType,
	1,
> = [(BlueprintItemType::Blueprint, 1000)];

pub(crate) const PACK_TYPE_MATERIAL_SPECIAL_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	SpecialItemType,
	4,
> = [
	(SpecialItemType::Dust, 999),
	(SpecialItemType::Unidentified, 1),
	(SpecialItemType::Fragment, 0),
	(SpecialItemType::ToolBox, 0),
];
// -----------------------------------------------

/// Probabilities for all PackType::Equipment options
pub(crate) const PACK_TYPE_EQUIPMENT_ITEM_PROBABILITIES: ProbabilitySlots<ItemType, 6> = [
	(ItemType::Pet, 50),
	(ItemType::Material, 450),
	(ItemType::Essence, 270),
	(ItemType::Equippable, 30),
	(ItemType::Blueprint, 0),
	(ItemType::Special, 200),
];

pub(crate) const PACK_TYPE_EQUIPMENT_PET_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<PetItemType, 3> =
	[(PetItemType::Pet, 0), (PetItemType::PetPart, 780), (PetItemType::Egg, 220)];

pub(crate) const PACK_TYPE_EQUIPMENT_MATERIAL_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	MaterialItemType,
	8,
> = [
	(MaterialItemType::Polymers, 135),
	(MaterialItemType::Electronics, 145),
	(MaterialItemType::PowerCells, 135),
	(MaterialItemType::Optics, 105),
	(MaterialItemType::Metals, 115),
	(MaterialItemType::Ceramics, 145),
	(MaterialItemType::Superconductors, 115),
	(MaterialItemType::Nanomaterials, 105),
];

pub(crate) const PACK_TYPE_EQUIPMENT_ESSENCE_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	EssenceItemType,
	5,
> = [
	(EssenceItemType::Glimmer, 740),
	(EssenceItemType::ColorSpark, 125),
	(EssenceItemType::GlowSpark, 125),
	(EssenceItemType::PaintFlask, 5),
	(EssenceItemType::GlowFlask, 5),
];

pub(crate) const PACK_TYPE_EQUIPMENT_EQUIPABLE_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	EquippableItemType,
	7,
> = [
	(EquippableItemType::ArmorBase, 760),
	(EquippableItemType::ArmorComponent1, 0),
	(EquippableItemType::ArmorComponent2, 0),
	(EquippableItemType::ArmorComponent3, 0),
	(EquippableItemType::WeaponVersion1, 120),
	(EquippableItemType::WeaponVersion2, 80),
	(EquippableItemType::WeaponVersion3, 40),
];

pub(crate) const PACK_TYPE_EQUIPMENT_BLUEPRINT_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	BlueprintItemType,
	1,
> = [(BlueprintItemType::Blueprint, 1000)];

pub(crate) const PACK_TYPE_EQUIPMENT_SPECIAL_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	SpecialItemType,
	4,
> = [
	(SpecialItemType::Dust, 999),
	(SpecialItemType::Unidentified, 1),
	(SpecialItemType::Fragment, 0),
	(SpecialItemType::ToolBox, 0),
];
// -----------------------------------------------

/// Probabilities for all PackType::Special options
pub(crate) const PACK_TYPE_SPECIAL_ITEM_PROBABILITIES: ProbabilitySlots<ItemType, 6> = [
	(ItemType::Pet, 495),
	(ItemType::Material, 250),
	(ItemType::Essence, 45),
	(ItemType::Equippable, 10),
	(ItemType::Blueprint, 0),
	(ItemType::Special, 200),
];

pub(crate) const PACK_TYPE_SPECIAL_PET_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<PetItemType, 3> =
	[(PetItemType::Pet, 1), (PetItemType::PetPart, 29), (PetItemType::Egg, 970)];

pub(crate) const PACK_TYPE_SPECIAL_MATERIAL_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	MaterialItemType,
	8,
> = [
	(MaterialItemType::Polymers, 135),
	(MaterialItemType::Electronics, 145),
	(MaterialItemType::PowerCells, 135),
	(MaterialItemType::Optics, 105),
	(MaterialItemType::Metals, 115),
	(MaterialItemType::Ceramics, 145),
	(MaterialItemType::Superconductors, 115),
	(MaterialItemType::Nanomaterials, 105),
];

pub(crate) const PACK_TYPE_SPECIAL_ESSENCE_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	EssenceItemType,
	5,
> = [
	(EssenceItemType::Glimmer, 998),
	(EssenceItemType::ColorSpark, 0),
	(EssenceItemType::GlowSpark, 0),
	(EssenceItemType::PaintFlask, 1),
	(EssenceItemType::GlowFlask, 1),
];

pub(crate) const PACK_TYPE_SPECIAL_EQUIPABLE_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	EquippableItemType,
	7,
> = [
	(EquippableItemType::ArmorBase, 970),
	(EquippableItemType::ArmorComponent1, 0),
	(EquippableItemType::ArmorComponent2, 0),
	(EquippableItemType::ArmorComponent3, 0),
	(EquippableItemType::WeaponVersion1, 15),
	(EquippableItemType::WeaponVersion2, 10),
	(EquippableItemType::WeaponVersion3, 5),
];

pub(crate) const PACK_TYPE_SPECIAL_BLUEPRINT_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	BlueprintItemType,
	1,
> = [(BlueprintItemType::Blueprint, 1000)];

pub(crate) const PACK_TYPE_SPECIAL_SPECIAL_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	SpecialItemType,
	4,
> = [
	(SpecialItemType::Dust, 999),
	(SpecialItemType::Unidentified, 1),
	(SpecialItemType::Fragment, 0),
	(SpecialItemType::ToolBox, 0),
];
// -----------------------------------------------

/// Probabilities for equipment slots
pub(crate) const ARMOR_SLOT_PROBABILITIES: ProbabilitySlots<SlotType, 7> = [
	(SlotType::Head, 140),
	(SlotType::Breast, 145),
	(SlotType::ArmFront, 140),
	(SlotType::ArmBack, 145),
	(SlotType::LegFront, 140),
	(SlotType::LegBack, 145),
	(SlotType::Tail, 145),
];

pub(crate) const WEAPON_SLOT_PROBABILITIES: ProbabilitySlots<SlotType, 2> =
	[(SlotType::WeaponFront, 500), (SlotType::WeaponBack, 500)];

/// Probabilities for pet type
pub(crate) const PET_TYPE_PROBABILITIES: ProbabilitySlots<PetType, 7> = [
	(PetType::TankyBullwog, 150),
	(PetType::FoxishDude, 150),
	(PetType::WierdFerry, 150),
	(PetType::FireDino, 150),
	(PetType::BigHybrid, 150),
	(PetType::GiantWoodStick, 150),
	(PetType::CrazyDude, 100),
];

/// Probabilities for equipment type
pub(crate) const EQUIPMENT_TYPE_PROBABILITIES: ProbabilitySlots<EquippableItemType, 7> = [
	(EquippableItemType::ArmorBase, 250),
	(EquippableItemType::ArmorComponent1, 150),
	(EquippableItemType::ArmorComponent2, 150),
	(EquippableItemType::ArmorComponent3, 150),
	(EquippableItemType::WeaponVersion1, 100),
	(EquippableItemType::WeaponVersion2, 100),
	(EquippableItemType::WeaponVersion3, 100),
];
