#import "../monokai_pro.typ": *

#let main(
  data,
  hovered_button: none,
  hovered_animation: 0.0,
) = {
  let transparency = 20%
  let hovered_transparency = 10%
  let darken = 50%

  let card_btn(lbl, fill, content) = {
    set box(
      fill: fill
        .transparentize(
          transparency
        )
        .darken(darken),
    )
    set box(
      fill: fill
        .transparentize(
          hovered_transparency
        )
        .darken(darken),
      // outset: 1em * hovered_animation,
      stroke: base7 + 2pt * hovered_animation,
      // radius: 2em * hovered_animation,
    ) if lbl == hovered_button
    set align(top)
    set text(size: 1.4em)

    [
      #box(
        width: 12em,
        height: 17em,
        inset: 2em,
      )[#content] #lbl
    ]
  }

  let button(lbl, fill, content) = {
    let box_fill = fill.transparentize(((1.0 - hovered_animation) * 100%))
    let text_fill = color.mix(
      (fill, ((1.0 - hovered_animation) * 100%)),
      (base0, hovered_animation * 100%),
    )

    set text(fill: text_fill) if lbl == hovered_button
    set box(fill: box_fill) if lbl == hovered_button

    [
      #box(
        inset: 0.9em,
        radius: 1em,
      )[#content] #lbl
    ]
  }

  box(width: 100%, height: 100%)[
    #place(center + horizon)[
      #stack(
        dir: ltr,
        spacing: 2%,
      )[
        #card_btn(<btn:1v1>, blue)[
          = 1 v 1
          #linebreak()
          Play against 1 player!
        ]
      ][
        #card_btn(<btn:2v2>, red)[
          = 2 v 2
          #linebreak()
          Team up and play against 2 players!
        ]
      ][
        #card_btn(<btn:3v3>, purple)[
          = 3 v 3
          #linebreak()
          Team up and play against 3 players!
        ]
      ]

      #align(right)[
        #button(<btn:cancel-matchmake>, red)[
          = Cancel
        ]
      ]
    ]
  ]
}
