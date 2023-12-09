type anime = {
  title: string,
  image: string,
  link: string,
}

type ongoing = array<anime>

open Json.Decode
let ongoing = array(
  object(field => {
    title: field.required(. "title", string),
    image: field.required(. "image", string),
    link: field.required(. "link", string),
  }),
)

type anime_list_eps = {
  title: string,
  link: string,
}
type anime_list = {
  title: string,
  image: string,
  episodes: array<anime_list_eps>,
}

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
