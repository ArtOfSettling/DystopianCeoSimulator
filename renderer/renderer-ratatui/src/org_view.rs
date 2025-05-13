use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};
use shared::GameStateSnapshot;

pub fn draw_org_view(
    f: &mut Frame,
    area: Rect,
    snapshot: &GameStateSnapshot,
    selected_index: usize,
) {
    let org_rows: Vec<Row> = snapshot
        .organizations
        .iter()
        .enumerate()
        .map(|(idx, org)| {
            let is_selected = idx == selected_index;

            // Pull VP name (or "Vacant")
            let vp_name = org
                .vp
                .and_then(|vp_id| org.employees.iter().find(|e| e.id == vp_id))
                .map(|vp| vp.name.clone())
                .unwrap_or_else(|| "Vacant".to_string());

            // Calculate aggregates
            let employee_count = org.employees.len();
            let avg_satisfaction: i32 = if employee_count > 0 {
                org.employees
                    .iter()
                    .filter_map(|emp| {
                        org.employees
                            .iter()
                            .find(|e| e.id == emp.id)
                            .map(|e| e.satisfaction)
                    })
                    .sum::<i32>()
                    / employee_count as i32
            } else {
                0
            };

            let total_salary: i32 = org
                .employees
                .iter()
                .filter_map(|emp| {
                    org.employees
                        .iter()
                        .find(|e| e.id == emp.id)
                        .map(|e| e.salary)
                })
                .sum();

            let cells = vec![
                Cell::from(org.name.clone()),
                Cell::from(vp_name),
                Cell::from(employee_count.to_string()),
                Cell::from(format!("{}%", avg_satisfaction)),
                Cell::from(format!("${}", total_salary)),
            ];

            let mut row = Row::new(cells);
            if is_selected {
                row = row.style(Style::default().add_modifier(Modifier::REVERSED));
            }
            row
        })
        .collect();

    let widths = [
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(5),
    ];
    let table = Table::new(org_rows, widths)
        .header(
            Row::new(vec![
                Cell::from("Org Name"),
                Cell::from("VP"),
                Cell::from("Employees"),
                Cell::from("Avg Sat"),
                Cell::from("Total Salary"),
            ])
            .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .block(
            Block::default()
                .title("Organizations")
                .borders(Borders::ALL),
        )
        .widths(&[
            Constraint::Percentage(25),
            Constraint::Percentage(20),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(25),
        ])
        .highlight_symbol(">> ");

    f.render_widget(table, area);
}
