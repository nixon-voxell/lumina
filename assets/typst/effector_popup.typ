#import "monokai_pro.typ": *

#let button_popup(body, progress: 0.0) = {
  set circle(fill: base1.transparentize(20%), stroke: base3 + 2pt, inset: 6pt)
  circle()[#body]
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

#let effector_popup(body) = {
  set align(horizon + center)
  set text(fill: base8)

  let body = text(body, size: 16pt, weight: "bold")
  let size = measure(body)

  place(dx: -size.width * 0.5, dy: -size.height - 10pt)[
    #box()[#body] <body>
  ]
}
