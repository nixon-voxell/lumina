#import "../monokai_pro.typ": *

#let lobby_list(lobbies) = [
  #box(width: 100%, height: 100%, inset: (x: 4.6%, y: 8%))[
    #set text(fill: base7, size: 24pt)
    = Lobbies

    #for (i, lobby) in lobbies.enumerate() [
      == Lobby \##i
      Player count: #lobby
    ]
  ]
]
