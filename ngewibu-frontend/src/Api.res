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
