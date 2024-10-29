#let red_box(body, width: 10pt, height: 10pt) = {
  box(fill: red, inset: 10pt)[#body]
}

#let main(width, height) = {
  // Convert float to length
  let width = (width * 1pt)
  let height = (height * 1pt)

  set text(size: 70pt)


  box(width: width, height: height, fill: none)[
    #place(center + horizon)[
      #red_box()[= Hello]
    ]
  ]
}
