@react.component
let make = () => {
  let url = RescriptReactRouter.useUrl()

  switch url.path {
  | list{} => <Home />
  | list{"anime", id} => <AnimeEpisode id />
  | list{"anime", "watch", id} => <Watch id />
  | _ => <NotFound />
  }
}
