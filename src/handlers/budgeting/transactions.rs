use chrono::Datelike;
use std::collections::{BTreeMap, HashMap};
use std::string::String;

use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};

use crate::types::common::AppError;
use crate::types::models::TransactionRow;
use crate::{
    handlers::util::parse_positive_i64,
    types::{
        common::{BotDialogue, DateFilter, HandleResult, TransactionKind},
        databases::{CategoriesDb, TransactionsDb},
    },
    utils::{
        markdown::escape_markdown_v2, statistics::amount_to_float,
        transactions::format_transaction_amount,
    },
};

pub async fn add_kind(
    kind: TransactionKind,
    _dialogue: BotDialogue,
    categories_db: &CategoriesDb,
    bot: Bot,
    msg: Message,
) -> HandleResult {
    let categories = categories_db.list(kind).await;

    let mut rows: Vec<Vec<InlineKeyboardButton>> = Vec::new();

    for category in categories {
        let id = category.id;
        let name = category.name;

        rows.push(vec![InlineKeyboardButton::callback(
            name.to_string(),
            format!("transaction:{}:add:category:{}:{}", kind.as_ref(), id, name),
        )]);
    }

    let keyboard = InlineKeyboardMarkup::new(rows);
    let message = escape_markdown_v2("✏️ Select category: ");

    bot.send_message(msg.chat.id, message)
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

pub async fn delete_last(bot: Bot, msg: Message, transactions_db: &TransactionsDb) -> HandleResult {
    let user_id_str = msg.chat.id.to_string();
    let parsed_user_id: i64 =
        parse_positive_i64(&bot, user_id_str.clone(), &user_id_str, "user id").await?;

    let last = transactions_db.get_last(parsed_user_id).await;

    let Some(last_tx) = last else {
        bot.send_message(user_id_str, "No transactions to delete.")
            .await?;

        return Ok(());
    };

    transactions_db.delete(last_tx.id).await?;

    let amount_str = format_transaction_amount(last_tx.amount, "+");
    let category_name = escape_markdown_v2(&last_tx.category_name);
    let description = escape_markdown_v2(&last_tx.description.clone());

    let confirmation = if description.is_empty() {
        format!("🗑️ Deleted: *{}* ({})", amount_str, category_name)
    } else {
        format!(
            "🗑️ Deleted: *{}* ({})\n{}",
            amount_str, category_name, description
        )
    };

    bot.send_message(user_id_str, escape_markdown_v2(&confirmation))
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;

    Ok(())
}

pub async fn search(
    bot: Bot,
    user_id: String,
    transactions_db: &TransactionsDb,
    search: &str,
) -> Result<Vec<TransactionRow>, AppError> {
    let user_id_str = user_id.to_string();
    let parsed_user_id: i64 =
        parse_positive_i64(&bot, user_id_str.clone(), &user_id_str, "user id").await?;

    let transactions = transactions_db
        .search_by_description(parsed_user_id, search, 5)
        .await;

    Ok(transactions)
}

pub async fn add_transaction(
    amount: i64,
    description: String,
    category_id: String,
    transactions_db: &TransactionsDb,
    bot: Bot,
    user_id: String,
    kind: TransactionKind,
) -> HandleResult {
    let parsed_user_id: i64 =
        parse_positive_i64(&bot, user_id.to_string(), &user_id, "user id").await?;

    let parsed_category_id: i64 =
        parse_positive_i64(&bot, user_id.to_string(), &category_id, "category id").await?;

    let signed_amount = kind.apply_sign(amount);

    transactions_db
        .add(
            signed_amount,
            Some(description.to_string()),
            parsed_user_id,
            parsed_category_id,
        )
        .await;

    let message = format!(
        "{} {} {} transaction added",
        kind,
        format_transaction_amount(signed_amount, "+"),
        description
    );

    bot.send_message(user_id, message).await?;

    Ok(())
}

#[derive(Debug, Clone)]
struct MonthlyTransaction {
    amount: f64,
    category_name: String,
    is_income: bool,
    description: String,
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

        if let Some(entries) = data.get(&cat) {
            for tx in entries {
                let amt_str = format_transaction_amount((tx.amount * 100.0) as i64, "");

                output.push_str(&format!("  {:<6} - {:<12}\n", amt_str, tx.description));
            }
        }

        output.push_str(" \n");
    }

    output
}

pub async fn list(
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
        let description = transaction.description.clone();
        let amount_f = amount_to_float(amount);

        monthly_transactions
            .entry((date.year(), date.month()))
            .or_default()
            .push(MonthlyTransaction {
                amount: amount_f.abs(),
                category_name: category_name.clone(),
                is_income: amount > 0,
                description: description.clone(),
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
                    category_name: tx.category_name.clone(),
                    is_income: tx.amount > 0,
                    description: tx.description.clone(),
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
                            .entry(tx.category_name.clone())
                            .or_default()
                            .push(tx.clone());
                    } else {
                        month_spending += tx.amount;
                        per_category_spending
                            .entry(tx.category_name.clone())
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
                    "\nMonth total income   {:>28}\nMonth total spending {:>28}\nMonth total          {:>28}\n",
                    format_transaction_amount((month_income * 100.0) as i64, ""),
                    format_transaction_amount((month_spending * 100.0) as i64, ""),
                    format_transaction_amount(((month_income - month_spending) * 100.0) as i64, "")
                ));
            }

            let total_sum = total_income_period - total_spending_period;

            output.push_str("===================================================\n");
            output.push_str(&format!(
                "Overall total income   {:>26}\nOverall total spending {:>26}\nOverall total          {:>26}\n",
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
