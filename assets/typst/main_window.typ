#let main_window(width, height, body) = [
  #let width = (width * 1pt)
  #let height = (height * 1pt)

  #box(
    width: width,
    height: height,
    // inset: (x: width * 4.6%, y: height * 8%),
  )[
    #for content in body {
      content
    }
  ]
]
