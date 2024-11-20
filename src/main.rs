use comfy_table::{modifiers::UTF8_ROUND_CORNERS, ContentArrangement, Table};
use math_parse::MathParse;
use std::io::Write;

fn greet() {
    println!("Welcome to quote calculator.");
    println!("Simply enter the cost of the parts on each lines");
    println!("Entering a non-neumerical value or pressing Enter twice will prompt me to calculate the total cost");
}

fn read_expr(idx: u8) -> Option<f64> {
    let mut input = String::new();
    print!("Part\t{}: $", idx);
    std::io::stdout().flush().unwrap();
    if std::io::stdin().read_line(&mut input).is_ok() {
        MathParse::parse(&input)
            .ok()
            .and_then(|t| t.solve_float(None).ok())
    } else {
        None
    }
}

fn calculate_total_cost(part_costs: &Vec<f64>) -> Vec<f64> {
    part_costs
        .iter()
        .copied()
        .map(|c| {
            if c < 200. {
                c + 140.
            } else if c <= 350. {
                c + 180.
            } else {
                c + 250.
            }
        })
        .collect()
}

fn draw_saperator() {
    println!("\x1b[33m{}\x1b[0m", "=".repeat(120));
}

fn draw_output(costs: &Vec<f64>, quotes: &Vec<f64>) {
    let mut table = Table::new();
    table
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec!["#", "Part cost", "Quote"]);
    for i in 0..costs.len() {
        table.add_row(vec![
            format!("{}", i + 1),
            format!("{:.02}", costs[i]),
            format!("{:.02}", quotes[i]),
        ]);
    }
    table.add_row(vec![
        comfy_table::Cell::new("Total".to_string()),
        comfy_table::Cell::new(""),
        comfy_table::Cell::new(format!("{:.02}", quotes.iter().copied().sum::<f64>()))
            .fg(comfy_table::Color::Black)
            .bg(comfy_table::Color::Green)
            .add_attribute(comfy_table::Attribute::Bold),
    ]);

    println!("{table}");

    println!(
        "Minimum deposit: {:.02}",
        quotes.iter().copied().sum::<f64>() * 0.5
    );

    draw_saperator();
}

fn main() {
    loop {
        greet();
        let mut part_costs = Vec::<f64>::new();
        let mut i = 1;
        loop {
            if let Some(cost) = read_expr(i) {
                part_costs.push(cost);
                i += 1;
            } else {
                break;
            }
        }
        let quotes = calculate_total_cost(&part_costs);
        draw_output(&part_costs, &quotes);
    }
}
