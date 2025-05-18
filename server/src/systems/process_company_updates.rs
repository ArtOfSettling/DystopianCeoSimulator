use crate::{InternalEventSender, NeedsStateUpdate};
use bevy::prelude::Res;
use shared::{Financials, InternalEvent, ServerGameState};

pub fn process_company_updates(
    needs_state_update: Res<NeedsStateUpdate>,
    internal_event_sender: Res<InternalEventSender>,
    server_game_state: Res<ServerGameState>,
) {
    if !needs_state_update.0 {
        return;
    }

    let (total_income, total_expenses, total_net_profit) = server_game_state
        .game_state
        .organizations
        .iter()
        .flat_map(|(_organization_id, organization)| {
            let f = &organization.financials;
            Some((
                f.this_weeks_income,
                f.this_weeks_expenses,
                f.this_weeks_net_profit,
            ))
        })
        .fold((0, 0, 0), |acc, (income, expenses, net_profit)| {
            (acc.0 + income, acc.1 + expenses, acc.2 + net_profit)
        });

    server_game_state
        .game_state
        .companies
        .iter()
        .for_each(|(company_id, company)| {
            internal_event_sender
                .tx_internal_events
                .try_send(InternalEvent::SetCompanyFinancials {
                    company_id: *company_id,
                    financials: Financials {
                        this_weeks_income: total_income,
                        this_weeks_expenses: total_expenses,
                        this_weeks_net_profit: total_net_profit,
                        actual_cash: company.financials.actual_cash,
                    },
                })
                .unwrap();
        });
}
