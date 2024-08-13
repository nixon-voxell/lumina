#import "typst/monokai_pro.typ": *

#set page(
  height: 1280pt,
  width: 1280pt,
  fill: base0,
)

#let parent(body) = {
  set text(fill: base8, size: 24pt)
  body
}

#let frame(body, stroke_color: base7) = {
  let fill_col = base0.mix((stroke_color, 20%))
  let text_col = base8.mix((stroke_color, 40%))
  box(stroke: stroke_color + 0.2em, fill: fill_col, inset: 1em)[
    #text(fill: text_col)[#body]
  ]
}

#let important_frame(body) = {
  frame(stroke_color: blue)[#body]
}

#let danger_frame(body) = {
  frame(stroke_color: red)[#body]
}

#parent()[
  #frame()[= Normal]\
  #important_frame()[= Important]\
  #danger_frame()[= Danger]
]
