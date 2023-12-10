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

#[derive(Serialize)]
pub struct MangaDetail {
    name: String,
    thumbnail: String,
    description: String,
    chapters: Vec<MangaChapter>,
}

#[derive(Serialize)]
pub struct MangaChapter {
    name: String,
    link: String,
}

pub async fn get_manga_detail(url: String) -> MangaDetail {
    let response = reqwest::get(url).await.expect("Failed to send request");

    let document = scraper::Html::parse_document(&response.text().await.expect("Failed to parse"));
    let title_selector = scraper::Selector::parse(".story-info-right>h1").unwrap();
    let image_selector = scraper::Selector::parse(".story-info-left>.info-image>img").unwrap();
    let desc_selector = scraper::Selector::parse(".panel-story-info-description").unwrap();
    let chapter_list_selector = scraper::Selector::parse("ul.row-content-chapter>li").unwrap();

    let title = document
        .select(&title_selector)
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap();
    let image = document
        .select(&image_selector)
        .next()
        .unwrap()
        .value()
        .attr("src")
        .unwrap();
    let desc = document
        .select(&desc_selector)
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap();

    let mut chapter_list: Vec<MangaChapter> = Vec::new();

    for chapter in document.select(&chapter_list_selector) {
        let item = scraper::Html::parse_fragment(&chapter.inner_html());
        let name_selector = scraper::Selector::parse("a.chapter-name").unwrap();

        let name = item
            .select(&name_selector)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap();
        let link = item
            .select(&name_selector)
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap();

        chapter_list.push(MangaChapter {
            name: name.to_owned(),
            link: link.to_owned(),
        })
    }

    MangaDetail {
        name: title.to_owned(),
        thumbnail: image.to_owned(),
        description: desc.to_owned(),
        chapters: chapter_list,
    }
}
