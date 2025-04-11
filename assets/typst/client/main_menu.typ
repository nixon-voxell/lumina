#import "../packages/suiji-0.3.0/src/lib.typ": *
#import "../monokai_pro.typ": *
#import "../utils.typ": *


#let connect_server(connection_msg, dummy_update) = {
  set text(size: 2em)

  place(center + horizon)[
    #set text(fill: base7)
    = #connection_msg

    #linebreak()

    #stack(dir: ltr)[
      #text(fill: yellow)[
        #button(
          lbl: <btn:reconnect>,
          inters: interactions(),
        )[Reconnect]
      ]
      #text(fill: red)[
        #button(
          lbl: <btn:exit-game>,
          inters: interactions(),
        )[Exit Game]
      ]
    ]
  ]
}


#let main_menu(
  connected,
  connection_msg,
  dummy_update,
) = {
  box(width: 100%, height: 100%, inset: 4em)[
    #if connected == false {
      connect_server(connection_msg, dummy_update)
      return
    }

    #place(center + horizon)[
      #text(fill: yellow, size: 7em, font: "IBrand")[= Lumina]

      #text(fill: green, size: 2em)[
        #button(
          lbl: <btn:play>,
          inters: interactions(),
        )[== Start Engine!]
      ]
    ]

    #place(right + bottom)[
      #box(height: 3em)[
        #button(
          lbl: <btn:settings>,
          inters: interactions(),
        )[== #emoji.gear Settings]
      ]

      #box(height: 3em)[
        #text(fill: red)[
          #button(
            lbl: <btn:exit-game>,
            inters: interactions(),
          )[== Abort]
        ]
      ]
    ]
  ]
}
