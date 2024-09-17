#import "../monokai_pro.typ": *
#import "../utils.typ": *

#set page(
  height: auto,
  width: auto,
  fill: base0,
)

#let matchmaking(width, height, player_count: 0) = {
  set text(fill: base7)

  let width = (width * 1pt)
  let height = (height * 1pt)

  box(
    width: width,
    height: height,
    inset: (x: width * 6%, y: height * 6%),
  )[
    #place(center + top)[
      #set text(size: 28pt)
      = Waiting for players (#player_count/6)
    ]
  ]
}
