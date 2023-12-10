use regex::Regex;
use serde::Serialize;

const BASE_URL: &str = "https://h.mangabat.com";

#[derive(Serialize)]
pub struct MangaListItem {
    name: String,
    link: String,
    thumbnail: String,
}

pub async fn get_manga_list() -> Vec<MangaListItem> {
    let url = format!("{}{}", BASE_URL, "/mangabat");

    let response = reqwest::get(url).await.expect("Failed to send request");

    let document = scraper::Html::parse_document(&response.text().await.expect("Failed to parse"));
    let manga_list_selector = scraper::Selector::parse(".content-homepage-item").unwrap();

    let mut manga_list: Vec<MangaListItem> = Vec::new();

    for manga in document.select(&manga_list_selector) {
        let item = scraper::Html::parse_fragment(&manga.inner_html());
        let name_selector = scraper::Selector::parse("h3.item-title>a").unwrap();
        let thumbnail_selector = scraper::Selector::parse("a.item-img>img").unwrap();
        let link_selector = scraper::Selector::parse("a.item-img").unwrap();

        let name = item
            .select(&name_selector)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap();
        let thumbnail = item
            .select(&thumbnail_selector)
            .next()
            .unwrap()
            .value()
            .attr("src")
            .unwrap();
        let link = item
            .select(&link_selector)
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap();

        manga_list.push(MangaListItem {
            name: name.to_owned(),
            thumbnail: thumbnail.to_owned(),
            link: link.to_owned(),
        })
    }

    manga_list
}
