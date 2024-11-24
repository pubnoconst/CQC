use comfy_table::{modifiers::UTF8_ROUND_CORNERS, ContentArrangement, Table};
use math_parse::MathParse;
use std::io::Write;

fn greet() {
    println!("Welcome to quote calculator.");
    println!("Simply enter the cost of the parts in each lines.");
    println!("Entering a non-neumerical value or pressing Enter twice will prompt me to calculate the total cost.");
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

fn calculate_quote(part_cost: f64) -> f64 {
    let over_200 = (part_cost >= 200.) as i32 as f64;
    let over_350 = (part_cost > 350.) as i32 as f64;

    part_cost + 140.0 + 40.0 * over_200 + 70.0 * over_350
}

#[cfg(test)]
mod tests {
    fn calculate_quote(part_cost: f64) -> f64 {
        if part_cost < 200. {
            part_cost + 140.
        } else if part_cost <= 350. {
            part_cost + 180. 
        } else {
            part_cost + 250. 
        }
    }
    
    #[test]
    fn tset() {
        for i in 0 .. 1000 {
            assert_eq!(self::calculate_quote(i as f64), super::calculate_quote(i as f64));
        }
    }
}

fn draw_saperator() {
    println!("\x1b[33m{}\x1b[0m", "=".repeat(120));
}

fn draw_output(costs: &[f64]) {
    let mut total_quote = 0.;
    let mut table = Table::new();
    table
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec!["#", "Part cost", "Quote"]);

    for (i, cost) in costs.iter().copied().enumerate() {
        let quote = calculate_quote(cost);
        total_quote += quote;
        table.add_row(vec![
            format!("{}", i + 1),
            format!("{:.02}", cost),
            format!("{:.02}", quote),
        ]);
    }

    table.add_row(vec![
        comfy_table::Cell::new("Total".to_string()),
        comfy_table::Cell::new(""),
        comfy_table::Cell::new(format!("{:.02}", total_quote))
            .fg(comfy_table::Color::Black)
            .bg(comfy_table::Color::Green)
            .add_attribute(comfy_table::Attribute::Bold),
    ]);

    println!("{table}");

    println!("Minimum deposit: ${:.02}", total_quote / 2.);

    draw_saperator();
}

fn main() {
    loop {
        greet();
        let mut part_costs = Vec::<f64>::new();
        let mut i = 1;
        while let Some(cost) = read_expr(i) {
            part_costs.push(cost);
            i += 1;
        }
        draw_output(&part_costs);
    }
}
