#import "../utils.typ": *
#import "../monokai_pro.typ": *

#let main(data) = {
  set text(size: 1em / data.scale)

  let color = base6.transparentize(60%).transparentize(100% * data.transparency)
  let raw_svg = read("/icons/right-arrow.svg")
  let raw_svg = raw_svg.replace("#ffffff", color.to-hex())

  box(width: 100%, height: 100%)[
    #place(
      center + horizon,
      dx: -data.camera_diff_x * 1pt / data.scale,
      dy: data.camera_diff_y * 1pt / data.scale,
      rotate(
        -data.rotation * 1rad,
        move(dx: 15em)[
          #rotate(data.rotation * 1rad)[
            #text(fill: color)[#calc.trunc(data.dist) m]
          ]
          #image(bytes(raw_svg), height: 2.5em)
        ],
      ),
    )
  ]
}
