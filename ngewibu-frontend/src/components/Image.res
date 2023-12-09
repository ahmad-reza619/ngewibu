type sizing = Small | Medium | Large

@react.component
let make = (~src: string, ~alt: string, ~size: sizing) => {
  let className = switch size {
  | Small => "w-12"
  | Medium => "w-24"
  | Large => "w-48"
  }
  <div className>
    <img src alt className="w-full" />
  </div>
}
