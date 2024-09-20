#import "../monokai_pro.typ": *
#import "../utils.typ": *

#set page(
  height: auto,
  width: auto,
  fill: base0,
)

#let main(width, height, lobbies) = {
  window(width, height)[
    #set text(fill: base7, size: 24pt)
    = Lobbies

    #for (i, lobby) in lobbies.enumerate() [
      == Lobby \##i
      Player count: #lobby
    ]
  ]
}

#let lobbies = (3, 2)
#main(1280, 720, lobbies)
