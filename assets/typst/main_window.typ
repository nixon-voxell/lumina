#let main_window(width, height, body) = [
  #let width = (width * 1pt)
  #let height = (height * 1pt)

  #box(
    width: width,
    height: height,
  )[
    #for content in body {
      content
    }
  ]
]
