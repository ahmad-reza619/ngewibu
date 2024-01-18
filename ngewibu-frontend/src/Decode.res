type anime = {
  title: string,
  image: string,
  link: string,
}

type ongoing = array<anime>

type anime_list_eps = {
  title: string,
  link: string,
}
type anime_list = {
  title: string,
  image: string,
  episodes: array<anime_list_eps>,
}

type manga_list_item = {
  name: string,
  link: string,
  thumbnail: string,
}

type manga_list = array<manga_list_item>

type manga_chapter = {
  name: string,
  link: string,
}

type manga_detail = {
  name: string,
  thumbnail: string,
  description: string,
  chapters: array<manga_chapter>,
}

type manga_read = {
  name: string,
  pages: array<string>,
}

open Json.Decode
let ongoing = array(
  object(field => {
    title: field.required(. "title", string),
    image: field.required(. "image", string),
    link: field.required(. "link", string),
  }),
)

let anime_list_eps = array(
  object(field => {
    title: field.required(. "title", string),
    link: field.required(. "link", string),
  }),
)

let anime_list = object(field => {
  title: field.required(. "title", string),
  image: field.required(. "image", string),
  episodes: field.required(. "episodes", anime_list_eps),
})

let manga_list_item = object(field => {
  name: field.required(. "name", string),
  link: field.required(. "link", string),
  thumbnail: field.required(. "thumbnail", string),
})

let manga_list = array(manga_list_item)

let manga_chapter = object(field => {
  name: field.required(. "name", string),
  link: field.required(. "link", string),
})

let manga_detail = object(field => {
  name: field.required(. "name", string),
  thumbnail: field.required(. "thumbnail", string),
  description: field.required(. "description", string),
  chapters: field.required(. "chapters", array(manga_chapter)),
})

let manga_read = object(field => {
  name: field.required(. "name", string),
  pages: field.required(. "pages", array(string)),
})
