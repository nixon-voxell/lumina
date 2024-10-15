#import "monokai_pro.typ": *

#let interactable_window(
  body,
  hovered_button: none,
  hovered_animation: 0.0,
) = [
  #let hovered_button = if hovered_button != none {
    hovered_button
  } else {
    label("")
  }

  #show hovered_button: body => {
    let box_fill = text.fill.transparentize(((1.0 - hovered_animation) * 100%))

    set text(
      fill: color.mix(
        (text.fill, ((1.0 - hovered_animation) * 100%)),
        (base0, hovered_animation * 100%),
      ),
    )

    box(
      fill: box_fill,
      radius: 10pt,
      outset: (hovered_animation * 6pt),
    )[#body]
  }

  #body
]

#let button(body, lbl: label) = {
  [#box(inset: 0.5em)[#body] #lbl]
}
