#import "../monokai_pro.typ": *
#import "../utils.typ": *

#let game_over(
  hovered_button,
  hovered_animation,
  local_team_index,
  team_names,
  team_scores,
) = {
  set text(fill: base7, size: 24pt)

  // Early return if there is nothing to show.
  // Team count must match!
  if team_names.len() <= 0 or team_names.len() != team_scores.len() {
    [
      = Something has gone terribly wrong!
      #button(lbl: <btn:main-menu>)[= Main Menu]
    ]
    return
  }

  let other_team_index = calc.rem(local_team_index + 1, team_names.len())
  let local_score = team_scores.at(local_team_index)
  let other_score = team_scores.at(other_team_index)
  let (result_col, result) = if local_score > other_score {
    (green, [Win!])
  } else if other_score > local_score {
    (red, [Lose...])
  } else {
    (yellow, [It's a tie...])
  }

  interactable_window(
    hovered_button: hovered_button,
    hovered_animation: hovered_animation,
  )[
    #box(width: 100%, height: 100%)[
      #place(center + horizon)[
        #set align(left)
        #box(
          width: team_scores.len() * 12em,
          inset: 40pt,
          fill: base0,
          stroke: result_col.transparentize(70%) + 6pt,
        )[
          #text(fill: result_col)[= #result]
          #line(length: 100%, stroke: base5 + 4pt)

          #grid(
            columns: (..team_scores.map(_ => 1fr), 1fr),
            row-gutter: 2em,
            column-gutter: 2em,
            [],
            ..team_names.map(it => [== Team #it]),
            [*Score*],
            ..team_scores.map(it => [#it])
          )

          #align(right)[
            #text(fill: purple)[
              #button(lbl: <btn:main-menu>)[= Main Menu]
            ]
          ]
        ]
      ]
    ]
  ]
}
