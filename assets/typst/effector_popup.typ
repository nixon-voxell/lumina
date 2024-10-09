#import "monokai_pro.typ": *

#let button_popup(body) = {
  circle(fill: base1.transparentize(20%), stroke: base3 + 2pt)[
    #set align(horizon + center)
    #body
  ]
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

#let effector_popup(x, y, body) = {
  let x = x * 1pt
  let y = y * 1pt


  // set text(fill: base7, size: 16pt)
  set text(fill: base8)
  let body = text(body, size: 16pt, weight: "bold")
  let size = measure(body)

  place(dx: x - size.width * 0.5, dy: y - size.height - 10pt)[
    #set align(horizon + center)
    #body
  ]
}
