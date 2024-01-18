module MangaCard = {
  @react.component
  let make = (~data: Decode.manga_list_item) => {
    let title = if data.name->Js.String2.length > 20 {
      data.name->Js.String.slice(~from=0, ~to_=20) ++ "..."
    } else {
      data.name
    }
    <div className="flex flex-col items-center">
      <a href={`/manga/detail?link=${data.link}`} target="_blank">
        <img src={data.thumbnail} className="w-40 h-56 rounded-sm" />
      </a>
      <p className="text-center w-40"> {title->React.string} </p>
    </div>
  }
}

module MangaList = {
  @react.component
  let make = () => {
    let (loading, setLoading) = React.useState(_ => true)
    let (mangas, setMangas) = React.useState(_ => [])
    let (error, setError) = React.useState(_ => None)
    React.useEffect0(() => {
      let fetchData = async () => {
        let result = await Api.getMangaList()
        switch result {
        | Ok(m) => {
            setMangas(_ => m)
            setLoading(_ => false)
          }
        | Error(err) => {
            Js.log(err)
            setLoading(_ => false)
            setError(_ => Some(err))
            setMangas(_ => [])
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
          {Belt.Array.map(mangas, data => {
            <MangaCard data key={data.name} />
          })->React.array}
        </div>
      }}
    </div>
  }
}

@react.component
let make = () => {
  <MangaList />
}
