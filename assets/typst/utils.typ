#import "monokai_pro.typ": *

#let window(width, height, body) = {
  let width = (width * 1pt)
  let height = (height * 1pt)

  box(
    width: width,
    height: height,
    inset: (x: width * 6%, y: height * 6%),
  )[#body]
}

#let interactable_window(
  width,
  height,
  body,
  hovered_button: none,
  hovered_animation: 0.0,
) = {
  window(width, height)[
    #let hovered_button = if hovered_button != none {
      hovered_button
    } else {
      label("")
    }

    #show hovered_button: body => {
      let box_fill = text.fill.transparentize((
        (1.0 - hovered_animation) * 100%
      ))

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
}

#let button(body, lbl: label) = {
  [#box(inset: 0.5em)[#body] #lbl]
}
