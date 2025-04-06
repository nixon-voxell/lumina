#import "../monokai_pro.typ": *
#import "../utils.typ": *

#let main(data, animate, closing, dummy_update) = {
  let set_label(lbl) = {
    if closing {
      label("")
    } else {
      lbl
    }
  }

  let (time0, time1, time2) = calculate_section_time(animate, 3)

  box(width: 100%, height: 100%)[
    #place(center + horizon)[
      #stack(
        dir: ltr,
        spacing: 2em,
        scale(90% + 10% * time0)[
          #text(size: 1.5em)[
            #let fill = yellow.transparentize(100% - 100% * time0)
            #card_button(
              lbl: set_label(<btn:assassin>),
              inters: interactions(),
              fill: fill,
            )[
              #text(fill: fill, underline[= Assassin])
              #linebreak()
              #set text(fill: fill.desaturate(80%))
              Fast and agile spaceship, specialized in stealth and precision strikes.
            ]
          ]
        ],
        scale(90% + 10% * time1)[
          #text(size: 1.5em)[
            #let fill = blue.transparentize(100% - 100% * time1)
            #card_button(
              lbl: set_label(<btn:defender>),
              inters: interactions(),
              fill: fill,
            )[
              #text(fill: fill, underline[= Defender])
              #linebreak()
              #set text(fill: fill.desaturate(80%))
              High durability spaceship, with strong shields for defense-focused gameplay.
            ]
          ]
        ],
      )


      #linebreak()
      #align(right)[
        #set text(fill: red.transparentize(100% - 100% * time2))
        #scale(90% + 10% * time2)[
          #button(
            lbl: <btn:cancel-spaceship>,
            inters: interactions(),
            disabled: closing,
          )[= Cancel]
        ]
      ]
    ]
  ]
}
