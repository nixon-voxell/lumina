#import "../packages/suiji-0.3.0/src/lib.typ": *
#import "../monokai_pro.typ": *
#import "../utils.typ": *

#let connect_server(connection_msg) = {
  set text(size: 24pt)

  place(center + horizon)[
    #set text(fill: base7)
    = #connection_msg

    #linebreak()

    #stack(dir: ltr)[
      #text(fill: yellow)[#button(lbl: <btn:reconnect>)[Reconnect]]
      #text(fill: red)[#button(lbl: <btn:exit-game>)[Exit Game]]
    ]
  ]
}

#let main_menu(
  hovered_button: none,
  hovered_animation: 0.0,
  connected: false,
  connection_msg: "Connecting to server...",
) = {
  interactable_window(
    hovered_button: hovered_button,
    hovered_animation: hovered_animation,
  )[
    #box(width: 100%, height: 100%, inset: (x: 4.6%, y: 8%))[
      #set text(fill: base7)

      #if connected == false {
        connect_server(connection_msg)
        return
      }

      #place(left + horizon)[
        #set text(size: 48pt)
        #pad(bottom: 40pt)[#text(fill: yellow)[= Lumina]]

        #move(dx: 2%)[
          #set text(size: 28pt)
          #text(fill: green)[#button(lbl: <btn:play>)[= Play]]\
          #text(fill: purple)[#button(lbl: <btn:luminators>)[= Luminators]]\
        ]
      ]

      #place(left + bottom)[

        #text(size: 16pt, fill: red.transparentize(40%))[
          #button(lbl: <btn:exit-game>)[= Exit Game]
        ]

        #text(size: 18pt)[
          #button(lbl: <btn:settings>)[#emoji.gear Settings]
        ]
      ]
    ]
  ]
}
