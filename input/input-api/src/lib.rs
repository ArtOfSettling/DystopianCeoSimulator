use bevy::prelude::Resource;

#[derive(Resource)]
pub struct InputResource {}

#[derive(Resource, Default, Debug)]
pub struct PendingPlayerInputAction(pub Option<PlayerInputAction>);

#[derive(Clone, Debug)]
pub enum PlayerInputAction {
    DoNothing,

    GoBack,

    LaunchPRCampaign,
    SelectEmployeeToFire,
    SelectEmployeeForRaise,

    MenuUp,
    MenuDown,
    MenuLeft,
    MenuRight,
}
