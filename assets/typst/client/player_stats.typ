#import "../utils.typ": *
#import "../monokai_pro.typ": *

#let main(players, scale, dummy_update) = {
  if players.len() == 0 {
    return
  }

  box(width: 100%, height: 100%)[
    #for player in players {
      let fill = if player.is_local_team { blue } else { red }
      if player.is_local_team == false {
        fill = fill.transparentize(player.transparency * 100%)
      }

      place(
        dx: player.x * 1pt,
        dy: player.y * 1pt,
      )[
        #place(center + horizon)[
          #rect(
            stroke: (
              paint: fill.transparentize(80%),
              thickness: 0.4em,
              dash: (array: (2em, 3em), phase: 1em),
            ),
            width: 5em,
            height: 5em,
          )
        ]
      ]

      place(
        dx: player.x * 1pt,
        dy: player.y * 1pt + 4em / scale,
      )[
        #place(center)[
          #text(
            fill: fill,
            player.name,
          )\
          #box(
            fill: fill.darken(40%).transparentize(60%),
            stroke: 0.15em + fill.darken(60%).transparentize(60%),
            width: 4em,
            height: 1em,
            radius: 0.3em,
            clip: true,
          )[
            #place(
              box(
                fill: fill.lighten(20%),
                width: 100% * player.health,
                height: 100%,
                radius: 0.3em,
              ),
            )
          ]
        ]
      ]
    }
  ]
}
