use std::collections::HashMap;

use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};

use crate::{
    handlers::util::parse_positive_i64,
    types::{
        common::{BotDialogue, DateFilter, HandleResult, TransactionKind},
        databases::{CategoriesDb, TransactionsDb},
    },
    utils::{
        markdown::escape_markdown_v2,
        statistics::amount_to_float,
        transactions::{
            day_key_from_timestamp, format_transaction_amount, format_transaction_date,
        },
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
        let name = category.name.clone();

        rows.push(vec![InlineKeyboardButton::callback(
            name,
            format!(
                "transaction:{}:add:category:{}",
                <TransactionKind as Into<&'static str>>::into(kind),
                id
            ),
        )]);
    }

    let keyboard = InlineKeyboardMarkup::new(rows);
    let message = escape_markdown_v2("✏️ Select category: ");

    bot.send_message(msg.chat.id, message)
        .reply_markup(keyboard)
        .await?;

    Ok(())
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

fn table(title: &str, data: &HashMap<String, Vec<(f64, String)>>, total: f64) -> String {
    let mut out = String::new();

    out.push_str(&format!("{:<28} {:>12} {:>6}\n", "Category", title, "(%)"));
    out.push_str("--------------------------------------------------\n");

    let mut cat_totals: Vec<(String, f64)> = data
        .iter()
        .map(|(cat, entries)| (cat.clone(), entries.iter().map(|(amt, _)| *amt).sum()))
        .collect();

    cat_totals.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for (cat, cat_total) in cat_totals {
        let pct = if total > 0.0 {
            (cat_total / total * 100.0).round() as i32
        } else {
            0
        };
        let amount_str = format_transaction_amount((cat_total * 100.0) as i64, "");

        out.push_str(&format!("{:<28} {:>12} {:>6}%\n", cat, amount_str, pct));

        if let Some(entries) = data.get(&cat) {
            for (amt, comment) in entries {
                let amt_str = format_transaction_amount((*amt * 100.0) as i64, "");
                out.push_str(&format!(
                    "  {:>10}  - {}\n",
                    amt_str,
                    escape_markdown_v2(comment)
                ));
            }
        }
    }

    out.push_str(&format!(
        "\nTotal {}: {}\n",
        title,
        format_transaction_amount((total * 100.0) as i64, "")
    ));

    out
}

pub async fn list(
    bot: Bot,
    user_id: String,
    transactions_db: &TransactionsDb,
    filter: DateFilter,
) -> HandleResult {
    let parsed_user_id: i64 =
        parse_positive_i64(&bot, user_id.to_string(), &user_id, "user id").await?;

    let mut transactions = transactions_db.list_filtered(parsed_user_id, filter).await;

    if transactions.is_empty() {
        bot.send_message(user_id, "No transactions found.").await?;

        return Ok(());
    }

    transactions.sort_by(|a, b| a.amount.cmp(&b.amount));

    let mut output = String::new();
    let mut current_day: Option<String> = None;
    let mut per_category_spending: HashMap<String, Vec<(f64, String)>> = HashMap::new();
    let mut per_category_income: HashMap<String, Vec<(f64, String)>> = HashMap::new();
    let mut total_spending = 0.0;
    let mut total_income = 0.0;

    for transaction in transactions {
        let timestamp = transaction.date;
        let amount = transaction.amount;
        let description = transaction.description.clone();
        let category = transaction.category.clone();

        let day_key = day_key_from_timestamp(timestamp);

        if current_day.as_ref() != Some(&day_key) {
            if current_day.is_some() {
                output.push_str("```\n");
                output.push_str(&table("Spending", &per_category_spending, total_spending));
                output.push_str(" \n");
                output.push_str(&table("Income  ", &per_category_income, total_income));
                output.push_str("```\n\n");

                per_category_spending.clear();
                per_category_income.clear();
                total_spending = 0.0;
                total_income = 0.0;
            }

            let formatted_date = escape_markdown_v2(&format_transaction_date(timestamp));

            output.push_str(&formatted_date);
            output.push('\n');
            current_day = Some(day_key);
        }

        let amount_f = amount_to_float(amount);

        if amount < 0 {
            total_spending += -amount_f;
            per_category_spending
                .entry(category.clone())
                .or_default()
                .push((-amount_f, description));
        } else {
            total_income += amount_f;
            per_category_income
                .entry(category.clone())
                .or_default()
                .push((amount_f, description));
        }
    }

    if current_day.is_some() {
        output.push_str("```\n");
        output.push_str(&table("Spending", &per_category_spending, total_spending));
        output.push_str(" \n");
        output.push_str(&table("Income  ", &per_category_income, total_income));
        output.push_str("```\n");
    }

    bot.send_message(user_id, output)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;

    Ok(())
}
