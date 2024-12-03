#import "monokai_pro.typ": *

#let perf_metrics(fps) = {
  set text(fill: base7)

  box(width: 100%, height: 100%)[
    #place(bottom + right)[
      #box(inset: 20pt)[
        #rect(fill: base1.transparentize(20%), outset: 10pt)[
          #align(left)[
            = Performance Metrics

            *FPS*: #fps
          ]
        ]
      ]
    ]
  ]
}
