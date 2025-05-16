use bevy::prelude::Resource;

#[derive(Resource)]
pub struct InputResource {}

#[derive(Resource, Default, Debug)]
pub struct PendingPlayerInputAction(pub Option<PlayerInputAction>);

#[derive(Clone, Debug)]
pub enum PlayerInputAction {
    DoNothing,
    Quit,

    LaunchPRCampaign,
    SelectEmployeeToFire,
    SelectEmployeeToHire,
    SelectEmployeeForRaise,
    SelectEmployeeForPromotionToVP,

    MenuUp,
    MenuDown,
    MenuLeft,
    MenuRight,
    MenuSelect,
    MenuBack,
    MenuChangeTab,
}
