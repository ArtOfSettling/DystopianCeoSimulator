use crate::{InternalEventSender, NeedsStateUpdate};
use bevy::prelude::Res;
use shared::{Budget, Financials, Initiative, InternalEvent, Perception, ServerGameState};

pub fn process_organization_updates(
    needs_state_update: Res<NeedsStateUpdate>,
    internal_event_sender: Res<InternalEventSender>,
    server_game_state: Res<ServerGameState>,
) {
    if !needs_state_update.0 {
        return;
    }

    for (organization_id, organization) in &server_game_state.game_state.organizations {
        let employees: Vec<_> = server_game_state
            .game_state
            .entities
            .values()
            .filter(|entity| {
                entity
                    .employment
                    .as_ref()
                    .map_or(false, |e| e.organization_id == *organization_id)
            })
            .collect();

        let productivity: u64 = employees
            .iter()
            .filter_map(|e| e.employment.as_ref())
            .map(|e| e.productivity as u64)
            .sum();

        let mut expenses: u64 = employees
            .iter()
            .filter_map(|e| e.employment.as_ref())
            .map(|e| e.salary as u64)
            .sum();

        let Budget {
            marketing,
            rnd,
            training,
        } = organization.budget;

        let total_budget = marketing + rnd + training;
        let can_afford = organization.financials.actual_cash >= total_budget as i32;

        if can_afford {
            expenses += total_budget as u64;
        }

        if marketing > 0 {
            internal_event_sender
                .tx_internal_events
                .try_send(InternalEvent::IncrementOrgPublicOpinion {
                    organization_id: *organization_id,
                    amount: 1,
                })
                .unwrap();
        }

        if rnd > 0 {
            internal_event_sender
                .tx_internal_events
                .try_send(InternalEvent::IncrementOrgReputation {
                    organization_id: *organization_id,
                    amount: 1,
                })
                .unwrap();
        }

        if training > 0 {
            for employee in &employees {
                internal_event_sender
                    .tx_internal_events
                    .try_send(InternalEvent::IncrementEmployeeSatisfaction {
                        employee_id: employee.id,
                        amount: 1,
                    })
                    .unwrap();
            }
        }

        let income = std::cmp::min(productivity, 10_000) as i16;
        let net_profit = income - expenses as i16;

        internal_event_sender
            .tx_internal_events
            .try_send(InternalEvent::SetOrgFinancials {
                organization_id: *organization_id,
                financials: Financials {
                    this_weeks_income: income as i32,
                    this_weeks_expenses: expenses as i32,
                    this_weeks_net_profit: net_profit as i32,
                    actual_cash: organization.financials.actual_cash + net_profit as i32,
                },
            })
            .unwrap();

        let (updated_initiatives, _initiative_change, opinion_change) =
            process_org_initiatives(&organization.initiatives);

        internal_event_sender
            .tx_internal_events
            .try_send(InternalEvent::SetOrgInitiatives {
                organization_id: *organization_id,
                initiatives: updated_initiatives.clone(),
            })
            .unwrap();

        if opinion_change.public_opinion_delta != 0 || opinion_change.reputation_delta != 0 {
            internal_event_sender
                .tx_internal_events
                .try_send(InternalEvent::SetOrgPublicOpinion {
                    organization_id: *organization_id,
                    perception: Perception {
                        public_opinion: organization.perception.public_opinion
                            + opinion_change.public_opinion_delta,
                        reputation: organization.perception.reputation
                            + opinion_change.reputation_delta,
                    },
                })
                .unwrap();
        }
    }
}

pub struct OrgInitiativeChange {
    #[allow(dead_code)]
    pub completed: Vec<Initiative>,
    #[allow(dead_code)]
    pub active: Vec<Initiative>,
}

pub struct OrgOpinionChange {
    pub reputation_delta: i16,
    pub public_opinion_delta: i16,
}

pub fn process_org_initiatives(
    initiatives: &[Initiative],
) -> (Vec<Initiative>, OrgInitiativeChange, OrgOpinionChange) {
    use Initiative::*;

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
