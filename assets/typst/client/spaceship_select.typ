#import "../monokai_pro.typ": *

#let spaceship_main(
  data,
  hovered_button: none,
  hovered_animation: 0.0,
) = {
  let transparency = 20%
  let hovered_transparency = 10%
  let darken = 50%

  let card_btn(lbl, fill, content) = {
    set box(fill: fill.transparentize(transparency).darken(darken))
    set box(
      fill: fill.transparentize(hovered_transparency).darken(darken),
      stroke: base7 + 2pt * hovered_animation,
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
        #card_btn(<btn:defender>, blue)[
          = Defender
          #linebreak()
          High durability spaceship with strong shields for defense-focused gameplay.
        ]
      ][
        #card_btn(<btn:assassin>, red)[
          = Assassin
          #linebreak()
          Fast and agile spaceship specialized in stealth and precision strikes.
        ]
      ]

      #align(right)[
        #button(<btn:cancel-spaceship>, red)[
          = Cancel
        ]
      ]
    ]
  ]
}