#import "monokai_pro.typ": *

#set page(
  width: auto,
  height: auto,
  fill: black,
  margin: 0pt,
)

#let parallelogram(
  length: 100pt,
  shear: 20pt,
  height: 30pt) = {
  polygon(
    fill: blue.lighten(80%),
    stroke: blue,
    (shear, 0pt),
    (length + shear, 0pt),
    (length, height),
    (0pt,  height),
  )
}

#let main(
  main_width: 1280pt,
  main_height: 720pt
) = {
  set text(fill: base8)

  box(
    width: main_width,
    height: main_height,
    inset: (x: main_width * 6%, y: main_height * 6%),
  )[
    #place(left + horizon)[
      #set text(size: 48pt)
      #box(stroke: red, outset: 50pt)[#text(fill: yellow)[= Side Effects]]

      #linebreak()

      #move(dx: 2%)[
        #set text(size: 32pt, fill: base7)
        #text(fill: green)[= Play]
        = Watch #text(fill: green, size: 20pt)[
          #emoji.triangle.r 4152 Live Now
        ]
        = Luminators
        = Tutorial

        #linebreak()

        #set text(size: 18pt, fill: red.mix((base0, 30%)))

        #parallelogram()
        = Exit Game
      ]
    ]

    #let percent = 1.0
    #let max_height = 300pt
    #let spacing = 0pt

    #place(horizon + right)[
      #align(center)[#stack(
        dir: ttb,
        spacing: -max_height * percent - spacing * 0.5,
        rect(height: max_height + spacing, width: 20pt + spacing, fill: white, stroke: white.transparentize(70%) + 10pt),
        rect(height: max_height * percent, width: 20pt, fill: gradient.linear(angle: 90deg, red, green, blue))
      )]
    ]

    #place(left + bottom)[
      #set text(size: 18pt)
      #emoji.gear Settings
    ]

    #let player_name = "Nixon"

    #place(right + top)[
      #parallelogram()
      #set text(size: 24pt, font: "Inter")

      #let size = 80pt
      #align(horizon)[
        #stack(
          dir: ltr,
          rect(fill: blue, width: size, height: size),
          box(
            width: 400pt,
            height: size,
            fill: base6.transparentize(80%),
            inset: 20pt,
          )[
            #stack(
              dir: ltr,
              spacing: 1fr,
              player_name,
              underline[View Profile],
            )
          ],
        )
      ]
    ]

    #let fps = 60
    #let elapsed_time = 1.23

    #place(right + bottom)[
      #set text(size: 18pt)

      #align(left)[
        = Performance Metrics
        FPS: #fps\
        Elapsed Time: #elapsed_time\
      ]
    ]

  ]
}

#main()
