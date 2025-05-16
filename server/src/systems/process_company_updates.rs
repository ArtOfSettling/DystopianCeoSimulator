use crate::{InternalEventSender, NeedsStateUpdate};
use bevy::prelude::{Query, Res};
use shared::{Company, Financials, InternalEvent, Organization};

pub fn process_company_updates(
    needs_state_update: Res<NeedsStateUpdate>,
    company: Res<Company>,
    internal_event_sender: Res<InternalEventSender>,
    organization_query: Query<&Organization>,
) {
    if !needs_state_update.0 {
        return;
    }

    let (total_income, total_expenses, total_net_profit) = organization_query
        .iter()
        .flat_map(|org| {
            let f = &org.financials;
            Some((
                f.this_weeks_income,
                f.this_weeks_expenses,
                f.this_weeks_net_profit,
            ))
        })
        .fold((0, 0, 0), |acc, (income, expenses, net_profit)| {
            (acc.0 + income, acc.1 + expenses, acc.2 + net_profit)
        });

    internal_event_sender
        .tx_internal_events
        .try_send(InternalEvent::SetCompanyFinancials {
            financials: Financials {
                this_weeks_income: total_income,
                this_weeks_expenses: total_expenses,
                this_weeks_net_profit: total_net_profit,
                actual_cash: company.financials.actual_cash,
            },
        })
        .unwrap();
}
