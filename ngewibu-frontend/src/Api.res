let getOngoing = async (): result<array<Decode.anime>, string> => {
  open Fetch

  try {
    let response = await fetch(
      "/api/ongoing",
      {
        method: #GET,
        headers: Headers.fromObject({"Content-Type": "application/json"}),
      },
    )

    let json = await response->Response.json

    json->Json.decode(Decode.ongoing)
  } catch {
  | Js.Exn.Error(err) =>
    switch Js.Exn.message(err) {
    | Some(msg) => Error("Error: " ++ msg)
    | None => Error("Unknown error")
    }
  }
}

let getMangaList = async (): result<Decode.manga_list, string> => {
  open Fetch

  try {
    let response = await fetch(
      "/api/manga-list",
      {
        method: #GET,
        headers: Headers.fromObject({"Content-Type": "application/json"}),
      },
    )

    let json = await response->Response.json

    json->Json.decode(Decode.manga_list)
  } catch {
  | Js.Exn.Error(err) =>
    switch Js.Exn.message(err) {
    | Some(msg) => Error("Error: " ++ msg)
    | None => Error("Unknown error")
    }
  }
}

let getAnimeEpisode = async (link: string): result<Decode.anime_list, string> => {
  open Fetch
  try {
    let response = await fetch(
      "/api/anime/" ++ link,
      {
        method: #GET,
        headers: Headers.fromObject({"Content-Type": "application/json"}),
      },
    )

    let json = await response->Response.json

    json->Json.decode(Decode.anime_list)
  } catch {
  | Js.Exn.Error(err) =>
    switch Js.Exn.message(err) {
    | Some(msg) => Error("Error: " ++ msg)
    | None => Error("Unknown error")
    }
  }
}

let getMangaDetail = async (payload): result<Decode.manga_detail, string> => {
  open Fetch
  try {
    let response = await fetch(
      "/api/manga",
      {
        method: #POST,
        body: payload->Js.Json.stringifyAny->Belt.Option.getExn->Body.string,
        headers: Headers.fromObject({"Content-Type": "application/json"}),
      },
    )

    let json = await response->Response.json

    json->Json.decode(Decode.manga_detail)
  } catch {
  | Js.Exn.Error(err) =>
    switch Js.Exn.message(err) {
    | Some(msg) => Error("Error: " ++ msg)
    | None => Error("Unknown error")
    }
  }
}

let readManga = async (payload): result<Decode.manga_read, string> => {
  open Fetch
  try {
    let response = await fetch(
      "/api/manga-chapter",
      {
        method: #POST,
        body: payload->Js.Json.stringifyAny->Belt.Option.getExn->Body.string,
        headers: Headers.fromObject({"Content-Type": "application/json"}),
      },
    )

    let json = await response->Response.json

    json->Json.decode(Decode.manga_read)
  } catch {
  | Js.Exn.Error(err) =>
    switch Js.Exn.message(err) {
    | Some(msg) => Error("Error: " ++ msg)
    | None => Error("Unknown error")
    }
  }
}
