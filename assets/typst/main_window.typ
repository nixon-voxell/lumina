#import "monokai_pro.typ": *

#let main_window(width, height, body, transparency) = {
  let width = (width * 1pt)
  let height = (height * 1pt)

  set text(
    size: calc.min(height, width) * 0.02,
    fill: base7,
    font: "Radio Canada",
  )

  box(
    width: width,
    height: height,
    fill: base0.transparentize(100% * transparency),
  )[
    #for content in body {
      place()[#content]
    }
  ]
}
