#import "../monokai_pro.typ": *
#import "../utils.typ": *

#let sandbox(
  room_id,
  dummy_update,
) = {
  box(width: 100%, height: 100%, inset: 2em)[
    #set text(fill: base7)

    #place(top + left)[
      = Sandbox
    ]

    #place(bottom + right, dy: 1.6em)[
      #text(fill: base6, size: 0.8em)[Room Id: #room_id]
    ]

    #place(top + right)[
      #set text(size: 0.7em)
      #text(fill: red)[
        #button(
          lbl: <btn:exit-sandbox>,
          inters: interactions(),
        )[= Exit]
      ]
    ]
  ]
}
