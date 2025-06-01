use crate::{GameClientInternalEvent, Instances};
use bevy::prelude::ResMut;
use shared::{Financials, InternalEvent};

pub fn process_company_updates(mut instances: ResMut<Instances>) {
    for (game_id, instance) in instances.active_instances.iter_mut() {
        if !instance.needs_state_update {
            return;
        }

        let (total_income, total_expenses, total_net_profit) = instance
            .instance_game
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

        instance
            .instance_game
            .game_state
            .companies
            .iter()
            .for_each(|(company_id, company)| {
                instance
                    .tx_internal_events
                    .try_send(GameClientInternalEvent {
                        game_id: *game_id,
                        internal_event: InternalEvent::SetCompanyFinancials {
                            company_id: *company_id,
                            financials: Financials {
                                this_weeks_income: total_income,
                                this_weeks_expenses: total_expenses,
                                this_weeks_net_profit: total_net_profit,
                                actual_cash: company.financials.actual_cash,
                            },
                        },
                    })
                    .unwrap();
            });
    }
}
