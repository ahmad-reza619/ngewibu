module ShowAnime = {
  @react.component
  let make = (~data: Decode.anime_list) => {
    <div className="container mx-auto mt-12">
      <div className="flex gap-4">
        <Image src={data.image} alt={data.title} size={Image.Medium} />
        <h1 className="text-3xl"> {React.string(data.title)} </h1>
      </div>
      <ul className="mt-12 flex gap-4 flex-col">
        {data.episodes
        ->Belt.Array.map(episode => {
          <li key={episode.link}>
            <a href={"/anime/watch/" ++ episode.link}> {React.string(episode.title)} </a>
          </li>
        })
        ->React.array}
      </ul>
    </div>
  }
}

@react.component
let make = (~id: string) => {
  let (data, setData) = React.useState(_ => None)

  React.useEffect1(() => {
    let fetchData = async () => {
      let result = await Api.getAnimeEpisode(id)
      switch result {
      | Ok(dat) => setData(_ => Some(dat))
      | Error(err) => Js.log(err)
      }
    }

    let _ = fetchData()

    None
  }, [id])

  switch data {
  | None => <div> {"Loading..."->React.string} </div>
  | Some(data) => <ShowAnime data />
  }
}
