module AnimeCard = {
  @react.component
  let make = (~data: Decode.anime) => {
    <div key={data.title} className="flex flex-col items-center">
      <a href={`/anime/${data.link}`} target="_blank">
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
        let result = await Api.getOngoing()
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
