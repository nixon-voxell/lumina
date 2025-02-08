#import "../monokai_pro.typ": *
#import "../utils.typ": *

#let sandbox(
  hovered_button,
  hovered_animation,
  room_id,
) = {
  interactable_window(
    hovered_button: hovered_button,
    hovered_animation: hovered_animation,
  )[
    #box(width: 100%, height: 100%, inset: (x: 4.6%, y: 8%))[
      #set text(fill: base7)

      #place(center + top)[
        #set text(size: 24pt)
        = Sandbox Mode
      ]

      #place(bottom + right)[
        #set text(size: 12pt)

        Room Id: #room_id
      ]

      #place(top + right)[
        #set text(size: 14pt)
        #text(fill: red)[#button(lbl: <btn:exit-sandbox>)[= Exit Sandbox]]
      ]
    ]
  ]
}