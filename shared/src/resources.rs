use bevy::prelude::Resource;
use uuid::Uuid;

#[derive(Resource)]
pub struct Company {
    pub revenue: i64,
    pub public_opinion: i32,
}

#[derive(Resource, Default, Debug)]
pub enum MenuState {
    #[default]
    AwaitingInput,
    ProcessingAction,
    DisplayingSummary,
}

#[derive(Resource, Debug)]
pub enum PendingAction {
    FireEmployee(Uuid),
    GiveRaise(Uuid, f64),
    LaunchPRCampaign,
    DoNothing,
}
