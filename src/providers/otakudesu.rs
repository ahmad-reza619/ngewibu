use serde::Serialize;

#[derive(Serialize)]
pub struct AnimeThumbnail {
    title: String,
    image: String,
    link: String,
}

pub async fn get_ongoing() -> Vec<AnimeThumbnail> {
    let response = reqwest::get("https://otakudesu.cam/ongoing-anime")
        .await
        .expect("Failed to send request");

    let document = scraper::Html::parse_document(&response.text().await.expect("Failed to parse"));
    let anime_list_selector = scraper::Selector::parse("div.venz>ul>li").unwrap();

    let mut ongoing: Vec<AnimeThumbnail> = Vec::new();

    for anime in document.select(&anime_list_selector) {
        let item = scraper::Html::parse_fragment(&anime.inner_html());
        let title_selector = scraper::Selector::parse("div.thumbz>h2.jdlflm").unwrap();
        let thumbnail_selector = scraper::Selector::parse("div.thumbz>img").unwrap();
        let link_selector = scraper::Selector::parse("div.thumb>a").unwrap();

        let title = item
            .select(&title_selector)
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
            .unwrap()[28..].trim_end_matches("/");
        println!("{}", link);
        ongoing.push(AnimeThumbnail {
            title: title.to_owned(),
            image: thumbnail.to_owned(),
            link: link.to_owned(),
        })
    }
    ongoing
}