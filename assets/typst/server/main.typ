#import "../monokai_pro.typ": *

#set page(
  height: auto,
  width: auto,
  fill: base0,
)

#let main(width, height, lobbies) = {
  set text(fill: base7, size: 24pt)

  let width = (width * 1pt)
  let height = (height * 1pt)

  box(
    width: width,
    height: height,
    inset: (x: width * 6%, y: height * 6%),
  )[
    = Lobbies

    #for (i, lobby) in lobbies.enumerate() [
      == Lobby \##i
      Player count: #lobby
    ]
  ]
}

#let lobbies = (3, 2)
#main(1280, 720, lobbies)
