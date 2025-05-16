use crate::Financials;
use bevy::prelude::Resource;
use uuid::Uuid;

#[derive(Resource)]
pub struct Company {
    pub public_opinion: i32,
    pub financials: Financials,
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
