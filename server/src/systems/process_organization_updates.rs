use crate::{InternalEventSender, NeedsStateUpdate};
use bevy::prelude::{Query, Res, With};
use shared::{
    Employed, Financials, InternalEntity, InternalEvent, OrgInitiative, Organization, Salary,
    Satisfaction,
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

        let (updated_initiatives, _initiative_change, opinion_change) =
            process_org_initiatives(&org.initiatives);

        internal_event_sender
            .tx_internal_events
            .try_send(InternalEvent::SetOrgInitiatives {
                organization_id: org.id,
                initiatives: updated_initiatives.clone(),
            })
            .unwrap();

        if opinion_change.public_opinion_delta != 0 || opinion_change.reputation_delta != 0 {
            internal_event_sender
                .tx_internal_events
                .try_send(InternalEvent::SetOrgPublicOpinion {
                    organization_id: org.id,
                    reputation_delta: opinion_change.reputation_delta,
                    public_opinion_delta: opinion_change.public_opinion_delta,
                })
                .unwrap();
        }
    });
}

pub struct OrgInitiativeChange {
    #[allow(dead_code)]
    pub completed: Vec<OrgInitiative>,
    #[allow(dead_code)]
    pub active: Vec<OrgInitiative>,
}

pub struct OrgOpinionChange {
    pub reputation_delta: i32,
    pub public_opinion_delta: i32,
}

pub fn process_org_initiatives(
    initiatives: &[OrgInitiative],
) -> (Vec<OrgInitiative>, OrgInitiativeChange, OrgOpinionChange) {
    use OrgInitiative::*;

    let (active, completed, reputation_delta, public_opinion_delta) = initiatives.iter().fold(
        (Vec::new(), Vec::new(), 0, 0),
        |(mut active, mut completed, rep, opinion), initiative| match initiative {
            Marketing { weeks_remaining } => {
                let new_weeks = weeks_remaining.saturating_sub(1);
                if new_weeks > 0 {
                    active.push(Marketing {
                        weeks_remaining: new_weeks,
                    });
                } else {
                    completed.push(initiative.clone());
                }
                (active, completed, rep, opinion + 2)
            }
            Training { weeks_remaining } => {
                let new_weeks = weeks_remaining.saturating_sub(1);
                if new_weeks > 0 {
                    active.push(Training {
                        weeks_remaining: new_weeks,
                    });
                } else {
                    completed.push(initiative.clone());
                }
                (active, completed, rep, opinion)
            }
            RnD { weeks_remaining } => {
                let new_weeks = weeks_remaining.saturating_sub(1);
                if new_weeks > 0 {
                    active.push(RnD {
                        weeks_remaining: new_weeks,
                    });
                } else {
                    completed.push(initiative.clone());
                }
                (active, completed, rep + 1, opinion)
            }
        },
    );

    let initiative_change = OrgInitiativeChange {
        completed,
        active: active.clone(),
    };
    let opinion_change = OrgOpinionChange {
        reputation_delta,
        public_opinion_delta,
    };

    (active, initiative_change, opinion_change)
}
