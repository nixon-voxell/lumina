#import "../monokai_pro.typ": *
#import "../utils.typ": *

#let settings(bgm_volume, vfx_volume, dummy_update) = {
  box(width: 100%, height: 100%, inset: (top: 7.25em, right: 2em))[
    #place(top + right)[
      #stack(
        dir: ltr,
        spacing: 0.25em,
        scale(75%)[
          #settings_button(
            fill: yellow.transparentize(50%),
            lbl: <btn:decrease_bgm>,
            inters: interactions(),
          )[#text(fill: yellow.lighten(20%), size: 1.75em)[= −]]
        ],
        scale(100%)[
          #parallelogram(
            height: 1.75em,
            width: 6em,
            slant: 0.25em,
            stroke: yellow.transparentize(50%) + 0.2em,
            fill: yellow.transparentize(60%),
            alignment: left + horizon
          )[
            #place(horizon, dx: 2em, text(fill: yellow.lighten(20%), size: 1em)[== #emoji.notes])
            #parallelogram(
              height: 1.75em,
              width: 6em * bgm_volume,
              slant: 0.25em,
              stroke: none,
              fill: yellow.transparentize(50%),
            )[]
          ]
        ],
        scale(75%)[
          #settings_button(
            fill: yellow.transparentize(50%),
            lbl: <btn:increase_bgm>,
            inters: interactions(),
          )[#text(fill: yellow.lighten(20%), size: 1.75em)[= +]]
        ],
      )
      #stack(
        dir: ltr,
        spacing: 0.25em,
        scale(75%)[
          #settings_button(
            fill: yellow.transparentize(50%),
            lbl: <btn:decrease_vfx>,
            inters: interactions(),
          )[#text(fill: yellow.lighten(20%), size: 1.75em)[= −]]
        ],
        scale(100%)[
          #parallelogram(
            height: 1.75em,
            width: 6em,
            slant: 0.25em,
            stroke: yellow.transparentize(50%) + 0.2em,
            fill: yellow.transparentize(60%),
            alignment: left + horizon
          )[
            #place(horizon, dx: 2em, text(fill: yellow.lighten(20%), size: 1em)[== #emoji.speaker.waves])
            #parallelogram(
              height: 1.75em,
              width: 6em * vfx_volume,
              slant: 0.25em,
              stroke: none,
              fill: yellow.transparentize(50%),
            )[]
          ]
        ],
        scale(75%)[
          #settings_button(
            fill: yellow.transparentize(50%),
            lbl: <btn:increase_vfx>,
            inters: interactions(),
          )[#text(fill: yellow.lighten(20%), size: 1.75em)[= +]]
        ],
      )
      
      #align(right)[
        #set text(size: 0.7em, fill: red)
        #scale(100%)[
          #button(
            lbl: <btn:close>,
            inters: interactions(),
          )[== #emoji.crossmark.box Close]
        ]
      ]
    ]
  ]
}