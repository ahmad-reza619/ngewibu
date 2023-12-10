use serde::Serialize;

#[derive(Serialize)]
pub struct AnimeThumbnail {
    title: String,
    image: String,
    link: String,
}

const BASE_URL: &str = "https://otakudesu.cam/";

pub async fn get_ongoing() -> Vec<AnimeThumbnail> {
    let url = format!("{}{}", BASE_URL, "ongoing-anime");

    let response = reqwest::get(url).await.expect("Failed to send request");

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
            .unwrap()[28..]
            .trim_end_matches('/');
        ongoing.push(AnimeThumbnail {
            title: title.to_owned(),
            image: thumbnail.to_owned(),
            link: link.to_owned(),
        })
    }
    ongoing
}

#[derive(Serialize)]
pub struct Anime {
    title: String,
    image: String,
    episodes: Vec<Episode>,
}

#[derive(Serialize)]
pub struct Episode {
    title: String,
    link: String,
}

pub async fn get_anime(id: String) -> Anime {
    let url = format!("{}{}{}", BASE_URL, "anime/", id);
    let response = reqwest::get(url).await.expect("Failed to send request");

    let document = scraper::Html::parse_document(&response.text().await.expect("Failed to parse"));
    let title_selector = scraper::Selector::parse(".venser>.jdlrx>h1").unwrap();
    let image_selector = scraper::Selector::parse("img.attachment-post-thumbnail").unwrap();
    let episode_selector = scraper::Selector::parse(".episodelist>ul>li").unwrap();

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

    let mut episodes: Vec<Episode> = Vec::new();

    for eps in document.select(&episode_selector) {
        let item = scraper::Html::parse_fragment(&eps.inner_html());
        let title_selector = scraper::Selector::parse("a").unwrap();
        let link_selector = scraper::Selector::parse("a").unwrap();

        let title = item
            .select(&title_selector)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap();
        let link = item
            .select(&link_selector)
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap()[30..]
            .trim_end_matches('/');

        episodes.push(Episode {
            title: title.to_owned(),
            link: link.to_owned(),
        })
    }

    let anime: Anime = Anime {
        title: title.to_owned(),
        image: image.to_owned(),
        episodes,
    };

    anime
}

// pub async fn watch(slug: String) -> Anime {
//     let url = format!("{}{}{}", BASE_URL, "episode/", slug);
//     let response = reqwest::get(url).await.expect("Failed to send request");
//
//     let document = scraper::Html::parse_document(&response.text().await.expect("Failed to parse"));
//     let video_selector = scraper::Selector::parse("video.jw-video.jw-reset").unwrap();
//
//     let video_el = document.select(&video_selector).next().unwrap();
// }
