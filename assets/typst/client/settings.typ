#import "../monokai_pro.typ": *
#import "../utils.typ": *

#let settings() = {
  box(width: 100%, height: 100%, inset: 4em, fill: black)[
    #place(center + top)[
      #text(fill: yellow, size: 4em, font: "IBrand")[= Settings]
    ]
    #place(center + horizon)[
      #text(fill: green, size: 2em)[
        BGM
      ]
      #linebreak()
      #linebreak()
      #text(fill: green, size: 2em)[
        VFX
      ]
    ]
    
    #place(center + bottom)[
      #box(height: 3em)[
        #text(fill: red)[
            #button(
            lbl: <btn:close>,
            inters: interactions(),
            )[== Close]
        ]
      ]
    ]
  ]
}
