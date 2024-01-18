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
      let result = await Api.readManga({
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
  | None => <div> {"Loading"->React.string} </div>
  | Some(data) =>
    <div>
      <h1 className="my-4 text-2xl"> {data.name->React.string} </h1>
      <hr />
      <div className="flex items-center flex-col">
        {data.pages
        ->Js.Array2.map(ch => {
          <div>
            <img src={ch} />
          </div>
        })
        ->React.array}
      </div>
    </div>
  }
}
