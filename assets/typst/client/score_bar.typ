#import "../monokai_pro.typ": *

#let score_display(team_score, col) = {
  rect(
    width: 3em,
    height: 3em,
    fill: col.darken(60%).transparentize(50%),
    radius: 4pt,
    stroke: col.darken(70%) + 0.1em,
  )[
    #align(horizon)[
      #text(fill: base8, size: 1.3em)[#team_score]
    ]
  ]
}

#let score_bar(score, max_score) = {
  let local_percent = (score / max_score) * 100%
  let other_percent = 100% - local_percent

  let grad_sharpness = 1%
  let local_grad = calc.max(local_percent - grad_sharpness, 0%)
  let other_grad = calc.min(local_percent + grad_sharpness, 100%)

  let friendly_color = blue.transparentize(70%)
  let enemy_color = red.transparentize(70%)

  box(width: 100%, height: 100%, inset: 2em)[
    #place(top + center)[
      #set align(horizon)
      #stack(
        dir: ltr,
        spacing: 1em,
        score_display(score, blue),
        box(
          width: 50%,
          height: 1em,
          fill: gradient.linear(
            space: rgb,
            (friendly_color, 0%),
            (friendly_color, local_grad),
            (enemy_color, other_grad),
            (enemy_color, 100%),
          ),
          outset: 0.2em,
        )[
          #place(left + horizon)[
            #box(
              width: local_percent,
              height: 100%,
              fill: blue,
            )
          ]
          #place(right + horizon)[
            #box(
              width: other_percent,
              height: 100%,
              fill: red,
            )
          ]

          #place(dx: local_percent - grad_sharpness * 0.5)[
            #box(
              width: 0.5em,
              height: 200%,
              fill: base8.transparentize(30%),
            )
          ]
        ],
        score_display(max_score - score, red),
      )
    ]
  ]
}
