#import "../monokai_pro.typ": *

#let lobby_list(lobbies) = [
  #set text(fill: base7, size: 24pt)
  = Lobbies

  #for (i, lobby) in lobbies.enumerate() [
    == Lobby \##i
    Player count: #lobby
  ]
]
