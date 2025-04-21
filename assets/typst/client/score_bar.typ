#import "../monokai_pro.typ": *
#import "../utils.typ": *

#let score_display(team_score, col) = {
  parallelogram(
    height: 2em,
    width: 3em,
    slant: 0.5em,
    stroke: none,
    fill: col.darken(80%).transparentize(30%),
  )[
    #text(fill: col.lighten(60%), size: 1.3em)[#team_score]
  ]
}

#let score_bar(score, max_score, dummy_update) = {
  let local_score = (score / max_score)
  let local_percent = local_score * 100%

  let grad_sharpness = 1%
  let local_grad = calc.max(local_percent - grad_sharpness, 0%)
  let other_grad = calc.min(local_percent + grad_sharpness, 100%)

  let friendly_color = blue.transparentize(70%)
  let enemy_color = red.transparentize(70%)

  let total_length = 30em

  let sin_time = calc.sin(elapsed-secs()) * 0.5 + 0.5

  box(width: 100%, height: 100%)[
    #place(top + center)[
      #set align(horizon)
      #stack(
        dir: ltr,
        spacing: -0.25em,
        score_display(score, blue),
        parallelogram(
          width: total_length,
          height: 1em,
          slant: 0.25em,
          fill: gradient.linear(
            (blue.darken(30%).transparentize(50%), 0%),
            (blue, lerp(sin_time, 0%, local_percent)),
            (blue.darken(30%).transparentize(50%), local_percent),
            (red.darken(30%).transparentize(50%), local_percent),
            (red, lerp(sin_time, 100%, local_percent)),
            (red.darken(30%).transparentize(50%), 100%),
          ),
          stroke: gradient.linear(
            (blue.lighten(60%).transparentize(60%), 0%),
            (blue.darken(60%).transparentize(60%), local_percent),
            (red.darken(60%).transparentize(60%), local_percent),
            (red.lighten(60%).transparentize(60%), 100%),
          )
            + 0.2em,
        )[],
        score_display(max_score - score, red),
      )
    ]
  ]
}
