#import "../monokai_pro.typ": *
#import "../utils.typ": *

#let main(
  data,
  hovered_button: none,
  hovered_animation: 0.0,
) = {
  let button(lbl, fill, content) = {
    set box(fill: fill)
    set box(
      fill: fill.opacify(30%),
      // outset: 1em * hovered_animation,
      stroke: base7 + 2pt * hovered_animation,
      // radius: 2em * hovered_animation,
    ) if lbl == hovered_button
    set align(top)

    [
      #box(
        width: 20%,
        height: 50%,
        inset: 2em,
      )[#content] #lbl
    ]
  }

  let transparency = 90%

  box(width: 100%, height: 100%)[
    #place(center + horizon)[
      #stack(
        dir: ltr,
        spacing: 2%,
      )[
        #button(<btn-1v1>, blue.transparentize(transparency))[
          = 1 v 1
          #linebreak()
          Play against 1 player!
        ]
      ][
        #button(<btn-2v2>, red.transparentize(transparency))[
          = 2 v 2
          #linebreak()
          Team up and play against 2 players!
        ]
      ][
        #button(<btn-3v3>, purple.transparentize(transparency))[
          = 3 v 3
          #linebreak()
          Team up and play against 3 players!
        ]
      ]
    ]
  ]
}
