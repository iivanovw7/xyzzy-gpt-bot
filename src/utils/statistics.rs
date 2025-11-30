use std::collections::HashMap;

use crate::utils::{markdown::escape_markdown_v2, transactions::format_transaction_amount};

pub fn amount_to_float(amount: i64) -> f64 {
    amount as f64 / 100.0
}

pub fn format_category_list(categories: &HashMap<String, f64>, total: f64) -> String {
    if categories.is_empty() {
        return "No entries.\n".to_string();
    }

    let mut out = String::new();

    for (cat, amount) in categories {
        let pct = if total > 0.0 {
            (*amount / total * 100.0).round()
        } else {
            0.0
        };

        let result = format!(
            "{} ({:.0}%) — {}\n",
            format_transaction_amount((*amount * 100.0) as i64, ""),
            pct,
            escape_markdown_v2(cat)
        );

        out.push_str(&escape_markdown_v2(&result));
    }

    out
}

pub fn format_sparkline_table(expenses: f64, income: f64, total: f64) -> String {
    let max_value = expenses.max(income);
    let expense_blocks = ((expenses / max_value) * 20.0).round() as usize;
    let income_blocks = ((income / max_value) * 20.0).round() as usize;

    let mut out = String::new();

    out.push_str(&escape_markdown_v2("Statistics in EUR\n\n"));
    out.push_str(&format!(
        "{:<10} {:>10} {:<20}\n",
        "Type", "Amount", "Spark"
    ));
    out.push_str(&format!("{:-<10} {:-<10} {:-<20}\n", "", "", ""));
    out.push_str(&format!(
        "{:<10} {:>10.2} {:<20}\n",
        "Expenses",
        expenses,
        "▇".repeat(expense_blocks)
    ));
    out.push_str(&format!(
        "{:<10} {:>10.2} {:<20}\n",
        "Income",
        income,
        "▇".repeat(income_blocks)
    ));
    out.push_str(&format!("{:<10} {:>10.2} {:<20}\n", "Total", total, ""));
    out.push_str("\nTotals:\n");
    out.push_str(&format!("Expenses: {:.2} EUR\n", expenses));
    out.push_str(&format!("Income:   {:.2} EUR\n", income));
    out.push_str(&format!("Total:    {:.2} EUR\n", total));

    escape_markdown_v2(&out)
}

pub fn format_totals(expenses: f64, income: f64, total: f64) -> String {
    let mut out = String::new();

    let total_expense = format!(
        "\nTotal expenses: {} EUR\n",
        format_transaction_amount((expenses * 100.0) as i64, "")
    );

    let total_income = format!(
        "Total income:   {} EUR\n",
        format_transaction_amount((income * 100.0) as i64, "")
    );

    let total_out = format!(
        "Total:          {} EUR\n",
        format_transaction_amount((total * 100.0) as i64, "")
    );

    out.push_str(&total_expense);
    out.push_str(&total_income);
    out.push_str(&total_out);

    escape_markdown_v2(&out)
}
