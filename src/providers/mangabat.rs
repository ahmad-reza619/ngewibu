use serde::Serialize;
use std::fmt::Debug;

const BASE_URL: &str = "https://h.mangabat.com";

#[derive(thiserror::Error, Debug)]
pub enum MangaBatError {
    #[error("Failed to send request")]
    RequestError(#[from] reqwest::Error),

    #[error("Failed to parse HTML")]
    ParsingError(#[from] scraper::error::SelectorErrorKind<'static>),

    #[error("Cannot select specified element")]
    FailedSelector,
}

#[derive(Serialize)]
pub struct MangaListItem {
    name: String,
    link: String,
    thumbnail: String,
}

pub async fn get_manga_list() -> Result<Vec<MangaListItem>, MangaBatError> {
    use MangaBatError::*;

    let url = format!("{}{}", BASE_URL, "/mangabat");

    let response = reqwest::get(url).await?;

    let document = scraper::Html::parse_document(&response.text().await?);
    let manga_list_selector = scraper::Selector::parse(".content-homepage-item")?;

    let mut manga_list: Vec<MangaListItem> = Vec::new();

    for manga in document.select(&manga_list_selector) {
        let item = scraper::Html::parse_fragment(&manga.inner_html());
        let name_selector = scraper::Selector::parse("h3.item-title>a")?;
        let thumbnail_selector = scraper::Selector::parse("a.item-img>img")?;
        let link_selector = scraper::Selector::parse("a.item-img")?;

        let name = item
            .select(&name_selector)
            .next()
            .and_then(|x| x.text().next())
            .ok_or(FailedSelector)?;
        let thumbnail = item
            .select(&thumbnail_selector)
            .next()
            .and_then(|x| x.value().attr("src"))
            .ok_or(FailedSelector)?;
        let link = item
            .select(&link_selector)
            .next()
            .and_then(|x| x.value().attr("href"))
            .ok_or(FailedSelector)?;

        manga_list.push(MangaListItem {
            name: name.to_owned(),
            thumbnail: thumbnail.to_owned(),
            link: link.to_owned(),
        })
    }

    Ok(manga_list)
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

pub async fn get_manga_detail(url: String) -> Result<MangaDetail, MangaBatError> {
    use MangaBatError::*;

    let response = reqwest::get(url).await?;

    let document = scraper::Html::parse_document(&response.text().await?);
    let title_selector = scraper::Selector::parse(".story-info-right>h1")?;
    let image_selector = scraper::Selector::parse(".story-info-left>.info-image>img")?;
    let desc_selector = scraper::Selector::parse(".panel-story-info-description")?;
    let chapter_list_selector = scraper::Selector::parse("ul.row-content-chapter>li")?;

    let title = document
        .select(&title_selector)
        .next()
        .and_then(|x| x.text().next())
        .ok_or(FailedSelector)?;
    let image = document
        .select(&image_selector)
        .next()
        .and_then(|x| x.value().attr("src"))
        .ok_or(FailedSelector)?;
    let desc = document
        .select(&desc_selector)
        .next()
        .and_then(|x| x.text().next())
        .ok_or(FailedSelector)?;

    let mut chapter_list: Vec<MangaChapter> = Vec::new();

    for chapter in document.select(&chapter_list_selector) {
        let item = scraper::Html::parse_fragment(&chapter.inner_html());
        let name_selector = scraper::Selector::parse("a.chapter-name")?;

        let name = item
            .select(&name_selector)
            .next()
            .and_then(|x| x.text().next())
            .ok_or(FailedSelector)?;
        let link = item
            .select(&name_selector)
            .next()
            .and_then(|x| x.value().attr("href"))
            .ok_or(FailedSelector)?;

        chapter_list.push(MangaChapter {
            name: name.to_owned(),
            link: link.to_owned(),
        })
    }

    Ok(MangaDetail {
        name: title.to_owned(),
        thumbnail: image.to_owned(),
        description: desc.to_owned(),
        chapters: chapter_list,
    })
}

#[derive(serde::Serialize)]
pub struct MangaChapterDetail {
    name: String,
    pages: Vec<String>,
}

async fn store_image(link: &str) -> Result<(), MangaBatError> {
    use std::fs::File;
    use std::io::copy;
    use std::path::Path;

    let folder_path = "./manga/";

    let file_name = link.split("/").last().unwrap_or("image.png");

    let file_path = format!("{}{}", folder_path, file_name);

    if !Path::new(folder_path).exists() {
        std::fs::create_dir(folder_path).expect("Failed to create folder");
    }

    let response = reqwest::get(link).await?;
    
    if response.status().is_success() {
        let mut file = File::create(file_path).expect("failed to create file");
        copy(&mut response.bytes().await?.as_ref(), &mut file).expect("Failed to write file");
        print!("{} downloaded\n", file_name);
    } else {
        eprintln!("Failed to download the image. Status code: {}", response.status());
    }

    Ok(())
}

pub async fn get_manga_chapter(url: String) -> Result<MangaChapterDetail, MangaBatError> {
    use MangaBatError::*;
    let response = reqwest::get(url).await?;

    let document = scraper::Html::parse_document(&response.text().await?);
    let title_selector = scraper::Selector::parse(".panel-chapter-info-top>h1")?;
    let pages_selector = scraper::Selector::parse(".container-chapter-reader>img")?;

    let title = document
        .select(&title_selector)
        .next()
        .and_then(|x| x.text().next())
        .ok_or(FailedSelector)?;

    let mut page_list: Vec<String> = Vec::new();
    let client = reqwest::Client::new();

    for pages in document.select(&pages_selector) {
        let img_src = pages.value().attr("src").ok_or(FailedSelector)?;
        let img = base64::encode(client.get(img_src).header("referer", "https://readmangabat.com/").send().await?.bytes().await?);
        page_list.push(format!("data:image/png;base64,{}", img));
    }

    Ok(MangaChapterDetail {
        name: title.to_owned(),
        pages: page_list,
    })
}
