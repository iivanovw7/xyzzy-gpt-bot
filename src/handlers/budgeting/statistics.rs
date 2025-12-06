use chrono::Datelike;
use std::collections::{BTreeMap, HashMap};

use crate::{
    handlers::util::parse_positive_i64,
    types::{
        common::{DateFilter, HandleResult},
        databases::TransactionsDb,
    },
    utils::{statistics::amount_to_float, transactions::format_transaction_amount},
};
use teloxide::prelude::*;

#[derive(Debug, Clone)]
struct MonthlyTransaction {
    amount: f64,
    category: String,
    is_income: bool,
}

type MonthlyMap = BTreeMap<(i32, u32), Vec<MonthlyTransaction>>;

fn table(title: &str, data: &HashMap<String, Vec<MonthlyTransaction>>, total: f64) -> String {
    let mut output = String::new();

    output.push_str("---------------------------------------------------\n");
    output.push_str(&format!("{:<28} {:>12} {:>6}\n", title, "Amount", "(%)"));
    output.push_str("---------------------------------------------------\n");

    let mut cat_totals: Vec<(String, f64)> = data
        .iter()
        .map(|(cat, entries)| (cat.clone(), entries.iter().map(|tx| tx.amount).sum()))
        .collect();

    cat_totals.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for (cat, cat_total) in cat_totals {
        let pct = if total > 0.0 {
            (cat_total / total * 100.0).round() as i32
        } else {
            0
        };
        let amount_str = format_transaction_amount((cat_total * 100.0) as i64, "");

        output.push_str(&format!("{:<28} {:>12} {:>6}%\n", cat, amount_str, pct));

        output.push_str(" \n");
    }

    output
}

pub async fn overview(
    bot: Bot,
    user_id: String,
    transactions_db: &TransactionsDb,
    filter: DateFilter,
) -> HandleResult {
    let parsed_user_id: i64 =
        parse_positive_i64(&bot, user_id.clone(), &user_id, "user id").await?;

    let transactions = transactions_db.list_filtered(parsed_user_id, filter).await;

    if transactions.is_empty() {
        bot.send_message(user_id.clone(), "No transactions found.")
            .await?;

        return Ok(());
    }

    let mut monthly_transactions: MonthlyMap = BTreeMap::new();

    for transaction in &transactions {
        let amount = transaction.amount;
        let date = transaction.date;
        let category_name = transaction.category_name.clone();
        let amount_f = amount_to_float(amount);

        monthly_transactions
            .entry((date.year(), date.month()))
            .or_default()
            .push(MonthlyTransaction {
                amount: amount_f.abs(),
                category: category_name.clone(),
                is_income: amount > 0,
            });
    }

    let table_output = match filter {
        DateFilter::Today | DateFilter::CurrentMonth | DateFilter::LastMonth => {
            let mut per_category_spending: HashMap<String, Vec<MonthlyTransaction>> =
                HashMap::new();
            let mut per_category_income: HashMap<String, Vec<MonthlyTransaction>> = HashMap::new();
            let mut total_spending = 0.0;
            let mut total_income = 0.0;

            for tx in &transactions {
                let amount_f = amount_to_float(tx.amount);
                let entry = MonthlyTransaction {
                    amount: amount_f.abs(),
                    category: tx.category_name.clone(),
                    is_income: tx.amount > 0,
                };
                if tx.amount < 0 {
                    total_spending += -amount_f;
                    per_category_spending
                        .entry(tx.category_name.clone())
                        .or_default()
                        .push(entry);
                } else {
                    total_income += amount_f;
                    per_category_income
                        .entry(tx.category_name.clone())
                        .or_default()
                        .push(entry);
                }
            }

            let title = match filter {
                DateFilter::Today => "Statistics for today",
                DateFilter::CurrentMonth => "Statistics for current month",
                DateFilter::LastMonth => "Statistics for last month",
                _ => "Statistics",
            };

            let mut output = format!("{}\n\n```\n", title);

            output.push_str(&table("Income  ", &per_category_income, total_income));
            output.push_str(" \n");
            output.push_str(&table("Spending", &per_category_spending, total_spending));
            output.push_str("\n---------------------------------------------------");
            output.push_str(&format!(
                "\nTotal income   {:>34}\nTotal spending {:>34}\nTotal          {:>34}\n",
                format_transaction_amount((total_income * 100.0) as i64, ""),
                format_transaction_amount((total_spending * 100.0) as i64, ""),
                format_transaction_amount(((total_income - total_spending) * 100.0) as i64, "")
            ));
            output.push_str("---------------------------------------------------\n");
            output.push_str("```\n");
            output
        }

        DateFilter::Last3Months | DateFilter::CurrentYear => {
            let mut total_spending_period = 0.0;
            let mut total_income_period = 0.0;
            let title = match filter {
                DateFilter::Last3Months => "Statistics for last 3 months",
                DateFilter::CurrentYear => "Statistics for current year",
                _ => "Statistics",
            };

            let mut output = format!("{}\n\n```\n", title);

            for ((year, month), txs) in &monthly_transactions {
                let mut per_category_spending: HashMap<String, Vec<MonthlyTransaction>> =
                    HashMap::new();
                let mut per_category_income: HashMap<String, Vec<MonthlyTransaction>> =
                    HashMap::new();
                let mut month_spending = 0.0;
                let mut month_income = 0.0;

                for tx in txs {
                    if tx.is_income {
                        month_income += tx.amount;
                        per_category_income
                            .entry(tx.category.clone())
                            .or_default()
                            .push(tx.clone());
                    } else {
                        month_spending += tx.amount;
                        per_category_spending
                            .entry(tx.category.clone())
                            .or_default()
                            .push(tx.clone());
                    }
                }

                total_spending_period += month_spending;
                total_income_period += month_income;

                output.push_str(&format!("Month: {:04}-{:02}\n", year, month));
                output.push_str(&table(
                    "Income category  ",
                    &per_category_income,
                    month_income,
                ));
                output.push_str(" \n");
                output.push_str(&table(
                    "Spending category",
                    &per_category_spending,
                    month_spending,
                ));
                output.push_str("\n---------------------------------------------------");
                output.push_str(&format!(
                    "\nMonth income   {:>34}\nMonth spending {:>34}\nMonth total    {:>34}\n",
                    format_transaction_amount((month_income * 100.0) as i64, ""),
                    format_transaction_amount((month_spending * 100.0) as i64, ""),
                    format_transaction_amount(((month_income - month_spending) * 100.0) as i64, "")
                ));
            }

            let total_sum = total_income_period - total_spending_period;

            output.push_str("===================================================\n");
            output.push_str(&format!(
                "Total income   {:>34}\nTotal spending {:>34}\nTotal          {:>34}\n",
                format_transaction_amount((total_income_period * 100.0) as i64, ""),
                format_transaction_amount((total_spending_period * 100.0) as i64, ""),
                format_transaction_amount((total_sum * 100.0) as i64, "")
            ));
            output.push_str("```\n");
            output
        }
    };

    bot.send_message(user_id, table_output)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;

    Ok(())
}
