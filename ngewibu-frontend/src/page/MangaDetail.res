@react.component
let make = () => {
  let url = RescriptReactRouter.useUrl()
  let (data, setData) = React.useState(() => None)

  let linkQuery =
    url.search
    ->Js.String2.split("&")
    ->Js.Array2.map(query => {
      let [key, value] = query->Js.String2.split("=")
      (key, value)
    })
    ->Js.Array2.find(((key, _)) => key == "link")

  let link = switch linkQuery {
  | Some((_, value)) => value
  | None => ""
  }

  React.useEffect1(() => {
    let fetchData = async () => {
      let result = await Api.getMangaDetail({
        "link": link,
      })
      switch result {
      | Ok(dat) => setData(_ => Some(dat))
      | Error(err) => Js.log(err)
      }
    }

    let _ = fetchData()

    None
  }, [link])

  switch data {
  | Some(data) =>
    <div>
      <div className="flex">
        <img src={data.thumbnail} />
        <div>
          <h1> {React.string(data.name)} </h1>
          <p> {React.string(data.description)} </p>
        </div>
      </div>
      <ul>
        {data.chapters
        ->Js.Array2.map(chapter => {
          <li key={chapter.name}>
            <a href={"/manga/read?link=" ++ chapter.link}> {React.string(chapter.name)} </a>
          </li>
        })
        ->React.array}
      </ul>
    </div>
  | None => <div> {"Loading"->React.string} </div>
  }
}
