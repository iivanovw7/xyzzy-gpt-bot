use crate::types::{
    databases::LinksDb,
    models::{LinkCategoryRow, LinkRow},
};
use sqlx::Row;

impl LinksDb {
    #[allow(clippy::too_many_arguments)]
    pub async fn add_link(
        &self,
        user_id: i64,
        url: &str,
        title: Option<&str>,
        description: Option<&str>,
        thumbnail_url: Option<&str>,
        category_id: Option<i64>,
        tags: Option<&str>,
    ) -> sqlx::Result<i64> {
        let result = sqlx::query!(
            r#"
            INSERT INTO links (user_id, url, title, description, thumbnail_url, category_id, tags)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            user_id,
            url,
            title,
            description,
            thumbnail_url,
            category_id,
            tags
        )
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get_link(&self, id: i64) -> sqlx::Result<Option<LinkRow>> {
        let row = sqlx::query(
            r#"
            SELECT
                l.id, l.user_id, l.url, l.title, l.description, l.thumbnail_url,
                l.category_id, lc.name as category_name, l.tags,
                l.created_at as created_at_unix,
                l.updated_at as updated_at_unix
            FROM links l
            LEFT JOIN link_categories lc ON l.category_id = lc.id
            WHERE l.id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| self.map_link_row(r)))
    }

    pub async fn get_link_by_url(&self, user_id: i64, url: &str) -> sqlx::Result<Option<LinkRow>> {
        let row = sqlx::query(
            r#"
            SELECT
                l.id, l.user_id, l.url, l.title, l.description, l.thumbnail_url,
                l.category_id, lc.name as category_name, l.tags,
                l.created_at as created_at_unix,
                l.updated_at as updated_at_unix
            FROM links l
            LEFT JOIN link_categories lc ON l.category_id = lc.id
            WHERE l.user_id = ? AND l.url = ?
            "#,
        )
        .bind(user_id)
        .bind(url)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| self.map_link_row(r)))
    }

    pub async fn update_link(
        &self,
        id: i64,
        title: Option<&str>,
        description: Option<&str>,
        thumbnail_url: Option<&str>,
        category_id: Option<i64>,
        tags: Option<&str>,
    ) -> sqlx::Result<bool> {
        let now = chrono::Utc::now().timestamp();
        let result = sqlx::query!(
            r#"
            UPDATE links
            SET title = COALESCE(?, title),
                description = COALESCE(?, description),
                thumbnail_url = COALESCE(?, thumbnail_url),
                category_id = COALESCE(?, category_id),
                tags = COALESCE(?, tags),
                updated_at = ?
            WHERE id = ?
            "#,
            title,
            description,
            thumbnail_url,
            category_id,
            tags,
            now,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_link(&self, id: i64) -> sqlx::Result<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM links
            WHERE id = ?
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn list_links(
        &self,
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> sqlx::Result<Vec<LinkRow>> {
        let rows = sqlx::query(
            r#"
            SELECT
                l.id, l.user_id, l.url, l.title, l.description, l.thumbnail_url,
                l.category_id, lc.name as category_name, l.tags,
                l.created_at as created_at_unix,
                l.updated_at as updated_at_unix
            FROM links l
            LEFT JOIN link_categories lc ON l.category_id = lc.id
            WHERE l.user_id = ?
            ORDER BY l.created_at DESC
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| self.map_link_row(r)).collect())
    }

    pub async fn count_links(&self, user_id: i64) -> sqlx::Result<i64> {
        let row = sqlx::query!(
            "SELECT COUNT(*) as count FROM links WHERE user_id = ?",
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.count)
    }

    pub async fn get_filtered_links(
        &self,
        user_id: i64,
        category: Option<&str>,
        tag: Option<&str>,
        search: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> sqlx::Result<(Vec<LinkRow>, i64)> {
        let mut query = r#"
            SELECT
                l.id, l.user_id, l.url, l.title, l.description, l.thumbnail_url,
                l.category_id, lc.name as category_name, l.tags,
                l.created_at as created_at_unix,
                l.updated_at as updated_at_unix
            FROM links l
            LEFT JOIN link_categories lc ON l.category_id = lc.id
            WHERE l.user_id = ?"#
            .to_string();

        let mut count_query = "SELECT COUNT(*) as count FROM links l LEFT JOIN link_categories lc ON l.category_id = lc.id WHERE l.user_id = ?".to_string();

        if let Some(c) = category {
            query.push_str(&format!(" AND lc.name = '{}'", c.replace("'", "''")));
            count_query.push_str(&format!(" AND lc.name = '{}'", c.replace("'", "''")));
        }

        if let Some(t) = tag {
            let like_tag = format!("'%{}%'", t.replace("'", "''"));
            query.push_str(&format!(" AND l.tags LIKE {}", like_tag));
            count_query.push_str(&format!(" AND l.tags LIKE {}", like_tag));
        }

        if let Some(s) = search {
            let s_safe = s.to_lowercase().replace("'", "''");
            let search_cond = format!(
                " AND (LOWER(l.title) LIKE '%{0}%' OR LOWER(l.description) LIKE '%{0}%' OR LOWER(l.url) LIKE '%{0}%' OR LOWER(l.tags) LIKE '%{0}%')",
                s_safe
            );
            query.push_str(&search_cond);
            count_query.push_str(&search_cond);
        }

        query.push_str(" ORDER BY l.created_at DESC LIMIT ? OFFSET ?");

        let rows = sqlx::query(&query)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;

        let row_count: (i64,) = sqlx::query_as(&count_query)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;

        Ok((
            rows.into_iter().map(|r| self.map_link_row(r)).collect(),
            row_count.0,
        ))
    }

    pub async fn list_tags(&self, user_id: i64) -> sqlx::Result<Vec<String>> {
        let rows = sqlx::query("SELECT tags FROM links WHERE user_id = ? AND tags IS NOT NULL")
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        let mut unique_tags = std::collections::BTreeSet::new();

        for row in rows {
            let tags: String = row.try_get("tags").unwrap_or_default();
            for tag in tags.split(',') {
                let trimmed = tag.trim();
                if !trimmed.is_empty() {
                    unique_tags.insert(trimmed.to_string());
                }
            }
        }

        Ok(unique_tags.into_iter().collect())
    }

    pub async fn add_category(&self, user_id: i64, name: &str) -> sqlx::Result<i64> {
        let _result = sqlx::query!(
            r#"
            INSERT INTO link_categories (user_id, name)
            VALUES (?, ?)
            ON CONFLICT(user_id, name) DO UPDATE SET name = excluded.name
            "#,
            user_id,
            name
        )
        .execute(&self.pool)
        .await?;

        let row = sqlx::query!(
            r#"SELECT id as "id!" FROM link_categories WHERE user_id = ? AND name = ?"#,
            user_id,
            name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.id)
    }

    pub async fn get_category_by_name(
        &self,
        user_id: i64,
        name: &str,
    ) -> sqlx::Result<Option<LinkCategoryRow>> {
        let row = sqlx::query(
            r#"
            SELECT id, user_id, name, created_at as created_at_unix
            FROM link_categories
            WHERE user_id = ? AND name = ?
            "#,
        )
        .bind(user_id)
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            let created_at_unix: i64 = r.try_get("created_at_unix").unwrap();
            LinkCategoryRow {
                id: r.try_get("id").unwrap(),
                user_id: r.try_get("user_id").unwrap(),
                name: r.try_get("name").unwrap(),
                created_at: chrono::DateTime::from_timestamp(created_at_unix, 0)
                    .map(|dt| dt.naive_utc())
                    .unwrap_or_default(),
            }
        }))
    }

    pub async fn list_categories(&self, user_id: i64) -> sqlx::Result<Vec<LinkCategoryRow>> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, name, created_at as created_at_unix
            FROM link_categories
            WHERE user_id = ?
            ORDER BY name ASC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let created_at_unix: i64 = r.try_get("created_at_unix").unwrap();
                LinkCategoryRow {
                    id: r.try_get("id").unwrap(),
                    user_id: r.try_get("user_id").unwrap(),
                    name: r.try_get("name").unwrap(),
                    created_at: chrono::DateTime::from_timestamp(created_at_unix, 0)
                        .map(|dt| dt.naive_utc())
                        .unwrap_or_default(),
                }
            })
            .collect())
    }

    pub async fn delete_category(&self, id: i64) -> sqlx::Result<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM link_categories
            WHERE id = ?
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    fn map_link_row(&self, r: sqlx::sqlite::SqliteRow) -> LinkRow {
        let created_at_unix: i64 = r.try_get("created_at_unix").unwrap();
        let updated_at_unix: i64 = r.try_get("updated_at_unix").unwrap();

        LinkRow {
            id: r.try_get("id").unwrap(),
            user_id: r.try_get("user_id").unwrap(),
            url: r.try_get("url").unwrap(),
            title: r.try_get("title").unwrap(),
            description: r.try_get("description").unwrap(),
            thumbnail_url: r.try_get("thumbnail_url").unwrap(),
            category_id: r.try_get("category_id").unwrap(),
            category_name: r.try_get("category_name").unwrap(),
            tags: r.try_get("tags").unwrap(),
            created_at: chrono::DateTime::from_timestamp(created_at_unix, 0)
                .map(|dt| dt.naive_utc())
                .unwrap_or_default(),
            updated_at: chrono::DateTime::from_timestamp(updated_at_unix, 0)
                .map(|dt| dt.naive_utc())
                .unwrap_or_default(),
        }
    }
}
