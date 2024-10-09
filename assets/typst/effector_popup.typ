#import "monokai_pro.typ": *

#let button_popup(body) = {
  circle()[
    #set align(horizon + center)
    #body
  ]
}

#let msg_popup(body) = {
  rect(radius: 4pt, inset: 10pt)[
    #set align(horizon + center)
    #body
  ]
}

#let effector_popup(x, y, body) = {
  let x = x * 1pt
  let y = y * 1pt


  // set text(fill: base7, size: 16pt)
  set text(fill: base7)
  let body = rect(text(body, size: 16pt), fill: base0)
  let size = measure(body)

  place(dx: x - size.width * 0.5, dy: y - size.height * 0.5)[
    #body
  ]
}
