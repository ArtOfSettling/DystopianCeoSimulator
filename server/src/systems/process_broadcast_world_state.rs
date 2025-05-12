use crate::NeedsWorldBroadcast;
use crate::systems::ServerEventSender;
use bevy::prelude::{Query, Res, ResMut};
use shared::{
    Employee, EmployeeSnapshot, GameStateSnapshot, InternalEntity, Money, Player, Reputation,
    Satisfaction, ServerEvent, Week,
};

pub fn process_broadcast_world_state(
    mut needs_broadcast: ResMut<NeedsWorldBroadcast>,
    query_money: Query<&Money>,
    query_rep: Query<&Reputation>,
    query_employees: Query<(&Employee, &Satisfaction)>,
    query_player: Query<(&Player, &Money, &Reputation, &Week, Option<&InternalEntity>)>,
    server_event_sender: Res<ServerEventSender>,
) {
    // only send state updates if the player entity has an internal_entity. I.E. an active connection.
    let (_, _, _, _, internal_entity) = query_player.single();
    if internal_entity.is_none() {
        return;
    }

    // only send state updates if the server decides we need to broadcast.
    if !needs_broadcast.0 {
        return;
    }
    needs_broadcast.0 = false;

    let money = query_money.single().0;
    let reputation = query_rep.single().0;

    let employees = query_employees
        .iter()
        .map(|(emp, sat)| EmployeeSnapshot {
            id: emp.id,
            name: emp.name.clone(),
            satisfaction: sat.0,
        })
        .collect();

    let snapshot = GameStateSnapshot {
        money,
        reputation,
        employees,
    };

    let _ = server_event_sender
        .tx_server_events
        .try_send(ServerEvent::FullState(snapshot.clone()));
}
