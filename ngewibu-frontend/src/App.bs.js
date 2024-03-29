// Generated by ReScript, PLEASE EDIT WITH CARE

import * as Home from "./page/Home.bs.js";
import * as Watch from "./page/Watch.bs.js";
import * as NotFound from "./page/NotFound.bs.js";
import * as AnimeList from "./page/AnimeList.bs.js";
import * as MangaList from "./page/MangaList.bs.js";
import * as MangaRead from "./page/MangaRead.bs.js";
import * as MangaDetail from "./page/MangaDetail.bs.js";
import * as AnimeEpisode from "./page/AnimeEpisode.bs.js";
import * as JsxRuntime from "react/jsx-runtime";
import * as RescriptReactRouter from "@rescript/react/src/RescriptReactRouter.bs.js";

function App(props) {
  var url = RescriptReactRouter.useUrl(undefined, undefined);
  var match = url.path;
  if (!match) {
    return JsxRuntime.jsx(Home.make, {});
  }
  switch (match.hd) {
    case "anime" :
        var match$1 = match.tl;
        if (!match$1) {
          return JsxRuntime.jsx(AnimeList.make, {});
        }
        var id = match$1.hd;
        if (!match$1.tl) {
          return JsxRuntime.jsx(AnimeEpisode.make, {
                      id: id
                    });
        }
        if (id !== "watch") {
          return JsxRuntime.jsx(NotFound.make, {});
        }
        var match$2 = match$1.tl;
        if (match$2.tl) {
          return JsxRuntime.jsx(NotFound.make, {});
        } else {
          return JsxRuntime.jsx(Watch.make, {
                      id: match$2.hd
                    });
        }
    case "manga" :
        var match$3 = match.tl;
        if (!match$3) {
          return JsxRuntime.jsx(MangaList.make, {});
        }
        switch (match$3.hd) {
          case "detail" :
              if (match$3.tl) {
                return JsxRuntime.jsx(NotFound.make, {});
              } else {
                return JsxRuntime.jsx(MangaDetail.make, {});
              }
          case "read" :
              if (match$3.tl) {
                return JsxRuntime.jsx(NotFound.make, {});
              } else {
                return JsxRuntime.jsx(MangaRead.make, {});
              }
          default:
            return JsxRuntime.jsx(NotFound.make, {});
        }
    default:
      return JsxRuntime.jsx(NotFound.make, {});
  }
}

var make = App;

export {
  make ,
}
/* Home Not a pure module */
