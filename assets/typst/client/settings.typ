#import "../monokai_pro.typ": *
#import "../utils.typ": *

#let settings() = {

  box(width: 100%, height: 100%, inset: (top: 6em, right: 2em))[
    #place(top + right)[
      #stack(
        dir: ltr,
        spacing: 2em,
        scale(100%)[
						#button(
						fill: yellow,
            lbl: <btn:bgm>,
            inters: interactions(),
          )[== #emoji.notes BGM]
        ],
        scale(100%)[
						#button(
						fill: yellow,
            lbl: <btn:vfx>,
            inters: interactions(),
          )[== #emoji.speaker.waves VFX]
        ],
      )
			
      #align(right)[
        #set text(fill: red)
        #scale(100%)[
          #button(
            lbl: <btn:close>,
            inters: interactions(),
          )[== #emoji.crossmark.box Close]
        ]
      ]
      #align(right)[
        #set text(fill: red)
        #scale(100%)[
          #button(
            lbl: <btn:leave>,
            inters: interactions(),
          )[== #emoji.arrow.r.filled Leave]
        ]
      ]
    ]
  ]
}