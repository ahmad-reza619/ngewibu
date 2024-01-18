@react.component
let make = () => {
  let url = RescriptReactRouter.useUrl()

  switch url.path {
  | list{} => <Home />
  | list{"anime"} => <AnimeList />
  | list{"anime", id} => <AnimeEpisode id />
  | list{"anime", "watch", id} => <Watch id />
  | list{"manga"} => <MangaList />
  | list{"manga", "detail"} => <MangaDetail />
  | list{"manga", "read"} => <MangaRead />
  | _ => <NotFound />
  }
}
