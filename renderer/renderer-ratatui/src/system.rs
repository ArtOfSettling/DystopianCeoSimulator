use bevy::app::AppExit;
use bevy::prelude::{EventWriter, Res, ResMut};
use input_api::PendingPlayerInputAction;
use renderer_api::RendererResource;
use shared::{GameStateSnapshot, PendingPlayerAction};

pub fn render_system(
    mut render_resource: ResMut<RendererResource>,
    game_state_snapshot: Res<GameStateSnapshot>,
    pending_player_input_action: ResMut<PendingPlayerInputAction>,
    pending_player_action: ResMut<PendingPlayerAction>,
    exit_writer: EventWriter<AppExit>,
) {
    render_resource.renderer.render(
        game_state_snapshot,
        pending_player_input_action,
        pending_player_action,
        exit_writer,
    );
}
