#import "monokai_pro.typ": *

#let button_popup(body, progress: 0.0) = {
  let button = text(size: 16pt)[
    #circle(
      fill: base1.transparentize(20%),
      stroke: base3 + 2pt,
      inset: 6pt,
    )[#scale(100% + progress * 20%)[#body]]
  ]
  let size = measure(button)
  let circum = (size.width * 0.5) * calc.pi * 2 * progress
  let interacted_color = blue

  text(
    stroke: (
      paint: interacted_color,
      thickness: 1pt,
      dash: (array: (circum, (1pt * calc.inf))),
    ),
    fill: color.mix(
      (base6, 100% - 100% * progress),
      (interacted_color, 100% * progress),
    ),
  )[#button]
  place(
    circle(
      stroke: (
        paint: blue,
        thickness: 2pt,
        dash: (array: (circum, (1pt * calc.inf))),
      ),
      width: size.width,
      height: size.height,
    ),
    dy: -size.height,
  )
}

#let msg_popup(body) = {
  rect(
    fill: base1.transparentize(20%),
    stroke: base3 + 2pt,
    radius: 4pt,
    inset: 10pt,
  )[
    #set align(horizon + center)
    #body
  ]
}

#let effector_popup(message, button, button_progress) = {
  set align(horizon + center)
  set text(fill: base8)

  let body = stack(
    dir: ltr,
    spacing: 10pt,
    if button != none {
      box(button_popup(button, progress: button_progress), inset: 10pt)
    },
    msg_popup(message),
  )

  let body = text(body, size: 16pt, weight: "bold")
  let size = measure(body)

  place(dx: -size.width * 0.5, dy: -size.height - 10pt)[
    #box(inset: 2pt)[#body] <body>
  ]
}
