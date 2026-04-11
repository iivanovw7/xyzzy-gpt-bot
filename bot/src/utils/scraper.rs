use reqwest::Client;
use scraper::{Html, Selector};
use url::Url;

#[derive(Debug, Default)]
pub struct LinkMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
}

pub async fn scrape_metadata(url: &str) -> anyhow::Result<LinkMetadata> {
    let client = Client::builder()
        .user_agent("facebookexternalhit/1.1 (+http://www.facebook.com/externalhit_uatext.php) Twitterbot/1.0 TelegramBot (like TwitterBot)")
        .build()?;

    let res = client.get(url).send().await?;
    let html_content = res.text().await?;
    let document = Html::parse_document(&html_content);

    let mut metadata = LinkMetadata::default();

    let title_selector = Selector::parse("title").unwrap();

    metadata.title = document
        .select(&title_selector)
        .next()
        .map(|el| el.inner_html().trim().to_string());

    if metadata.title.is_none() {
        let og_title_selector = Selector::parse("meta[property='og:title']").unwrap();

        metadata.title = document
            .select(&og_title_selector)
            .next()
            .and_then(|el| el.value().attr("content").map(|s| s.trim().to_string()));
    }

    let desc_selector = Selector::parse("meta[name='description']").unwrap();

    metadata.description = document
        .select(&desc_selector)
        .next()
        .and_then(|el| el.value().attr("content").map(|s| s.trim().to_string()));

    if metadata.description.is_none() {
        let og_desc_selector = Selector::parse("meta[property='og:description']").unwrap();

        metadata.description = document
            .select(&og_desc_selector)
            .next()
            .and_then(|el| el.value().attr("content").map(|s| s.trim().to_string()));
    }

    let og_image_selector = Selector::parse("meta[property='og:image']").unwrap();

    metadata.thumbnail_url = document
        .select(&og_image_selector)
        .next()
        .and_then(|el| el.value().attr("content").map(|s| s.to_string()));

    if metadata.thumbnail_url.is_none() {
        let og_image_name_selector = Selector::parse("meta[name='og:image']").unwrap();
        metadata.thumbnail_url = document
            .select(&og_image_name_selector)
            .next()
            .and_then(|el| el.value().attr("content").map(|s| s.to_string()));
    }

    if metadata.thumbnail_url.is_none() {
        let twitter_image_selector = Selector::parse("meta[name='twitter:image']").unwrap();

        metadata.thumbnail_url = document
            .select(&twitter_image_selector)
            .next()
            .and_then(|el| el.value().attr("content").map(|s| s.to_string()));
    }

    if metadata.thumbnail_url.is_none() {
        let twitter_image_prop_selector =
            Selector::parse("meta[property='twitter:image']").unwrap();

        metadata.thumbnail_url = document
            .select(&twitter_image_prop_selector)
            .next()
            .and_then(|el| el.value().attr("content").map(|s| s.to_string()));
    }

    if metadata.thumbnail_url.is_none() {
        let image_src_selector = Selector::parse("link[rel='image_src']").unwrap();

        metadata.thumbnail_url = document
            .select(&image_src_selector)
            .next()
            .and_then(|el| el.value().attr("href").map(|s| s.to_string()));
    }

    if let Some(thumb_url) = &metadata.thumbnail_url {
        if let Ok(parsed_base_url) = Url::parse(url) {
            if let Ok(absolute_thumb_url) = parsed_base_url.join(thumb_url) {
                metadata.thumbnail_url = Some(absolute_thumb_url.to_string());
            }
        }
    }

    Ok(metadata)
}
