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
      radius: 0.1em,
      outset: (hovered_animation * 0.25em),
    )[#body]
  }

  #body
]

#let button(body, lbl: label, fill: white) = {
  let raw_svg = read("../icons/button01.svg")
  raw_svg = raw_svg.replace(
    "#fff",
    fill.to-hex(),
  )
  let background = image(bytes(raw_svg))
  [
    #box()[
      #background
      #place(center + horizon, body)
    ] #lbl
  ]
}

#let lerp(x, low, high) = {
  let diff = high - low
  x * diff + low
}
