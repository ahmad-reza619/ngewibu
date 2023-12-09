// Generated by ReScript, PLEASE EDIT WITH CARE

import * as Home from "./page/Home.bs.js";
import * as Watch from "./page/Watch.bs.js";
import * as NotFound from "./page/NotFound.bs.js";
import * as AnimeEpisode from "./page/AnimeEpisode.bs.js";
import * as JsxRuntime from "react/jsx-runtime";
import * as RescriptReactRouter from "@rescript/react/src/RescriptReactRouter.bs.js";

function App(props) {
  var url = RescriptReactRouter.useUrl(undefined, undefined);
  var match = url.path;
  if (!match) {
    return JsxRuntime.jsx(Home.make, {});
  }
  if (match.hd !== "anime") {
    return JsxRuntime.jsx(NotFound.make, {});
  }
  var match$1 = match.tl;
  if (!match$1) {
    return JsxRuntime.jsx(NotFound.make, {});
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
}

var make = App;

export {
  make ,
}
/* Home Not a pure module */
