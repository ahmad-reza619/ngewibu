type anime = {
  title: string,
  image: string,
  link: string,
}

type ongoing = array<anime>

module Decode = {
  open Json.Decode
  let ongoing = array(
    object(field => {
      title: field.required(. "title", string),
      image: field.required(. "image", string),
      link: field.required(. "link", string),
    }),
  )
}

let getOngoing = async (): result<array<anime>, string> => {
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
    | Js.Exn.Error(err) => switch Js.Exn.message(err) {
      | Some(msg) => {
        Error("Error: " ++ msg)
      }
      | None => {
        Error("Unknown error")
        }
      }
  }
}

module AnimeCard = {
  @react.component
  let make = (~data: anime) => {
    <div key={data.title} className="flex flex-col items-center">
      <a href={data.link} target="_blank">
        <img src={data.image} className="w-40 h-56 rounded-sm" />
      </a>
      <p className="text-center w-40"> {React.string(data.title)} </p>
    </div>
  }
}

module OngoingList = {
  @react.component
  let make = () => {
    let (loading, setLoading) = React.useState(_ => true)
    let (animes, setAnimes) = React.useState(_ => [])
    let (error, setError) = React.useState(_ => None)
    React.useEffect0(() => {
      let fetchData = async () => {
        let result = await getOngoing()
        switch result {
        | Ok(animes) => {
            setAnimes(_ => animes)
            setLoading(_ => false)
          }
        | Error(err) => {
            Js.log(err)
            setLoading(_ => false)
            setError(_ => Some(err))
          }
        }
      }

      let _ = fetchData()

      None
    })

    <div className="container mx-auto mt-12">
      <h1 className="text-3xl"> {React.string("Ngewibu")} </h1>
      {switch (loading, error) {
      | (true, _) => <p className="text-center mt-6"> {React.string("Loading...")} </p>
      | (false, Some(err)) => <p className="text-center mt-6"> {React.string(err)} </p>
      | (false, None) =>
        <div className="flex gap-3 mt-6 flex-wrap">
          {Belt.Array.map(animes, data => {
            <AnimeCard data key={data.title} />
          })->React.array}
        </div>
      }}
    </div>
  }
}

@react.component
let make = () => {
  <OngoingList />
}
