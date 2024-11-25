#import "monokai_pro.typ": *

#let main_window(width, height, body, transparency) = [
  #let width = (width * 1pt)
  #let height = (height * 1pt)

  #box(
    width: width,
    height: height,
    fill: base0.transparentize(100% * transparency),
  )[
    #for content in body {
      place()[#content]
    }
  ]
]
