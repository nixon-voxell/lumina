#import "../utils.typ": *
#import "../monokai_pro.typ": *

#let main(players, scale, dummy_update) = {
  if players.len() == 0 {
    return
  }

  box(width: 100%, height: 100%)[
    #for player in players {
      place(
        dx: player.x * 1pt,
        dy: player.y * 1pt + 4em / scale,
      )[
        #place(center)[
          #text(fill: if player.is_local { blue } else { red }, player.name)
        ]
      ]
    }
  ]
}
