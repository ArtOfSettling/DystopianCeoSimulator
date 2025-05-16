use crate::{InternalEventSender, NeedsStateUpdate};
use bevy::prelude::{Query, Res, With};
use shared::{
    Employed, Financials, InternalEntity, InternalEvent, Organization, Salary, Satisfaction,
};

pub fn process_organization_updates(
    needs_state_update: Res<NeedsStateUpdate>,
    internal_event_sender: Res<InternalEventSender>,
    organization_query: Query<&Organization>,
    employee_query: Query<(&InternalEntity, &Employed, &Salary, &Satisfaction), With<Employed>>,
) {
    if !needs_state_update.0 {
        return;
    }

    organization_query.iter().for_each(|org| {
        // Collect all employees for this organization
        let employees: Vec<_> = employee_query
            .iter()
            .filter(|(_, employed, _, _)| employed.owner_id == org.id)
            .collect();

        let productivity: i32 = employees
            .iter()
            .map(|(_, _, _, satisfaction)| satisfaction.0)
            .sum();

        let expenses: i32 = employees.iter().map(|(_, _, salary, _)| salary.0).sum();

        let productivity_multiplier = 125;
        let income = productivity * productivity_multiplier;
        let net_profit = income - expenses;

        internal_event_sender
            .tx_internal_events
            .try_send(InternalEvent::SetOrgFinancials {
                organization_id: org.id,
                financials: Financials {
                    this_weeks_income: income,
                    this_weeks_expenses: expenses,
                    this_weeks_net_profit: net_profit,
                    actual_cash: org.financials.actual_cash + net_profit,
                },
            })
            .unwrap();
    });
}
