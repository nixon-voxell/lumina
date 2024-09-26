#import "../packages/suiji-0.3.0/src/lib.typ": *
#import "../monokai_pro.typ": *
#import "../utils.typ": *

#set page(
  height: auto,
  width: auto,
  fill: base0,
)

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
  width,
  height,
  hovered_button: none,
  hovered_animation: 0.0,
  connected: false,
  connection_msg: "Disconnected...",
) = {
  interactable_window(
    width,
    height,
    hovered_button: hovered_button,
    hovered_animation: hovered_animation,
  )[
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
        #button(lbl: <btn:tutorial>)[= Tutorial]\

        #set text(size: 16pt, fill: red.transparentize(40%))
        #button(lbl: <btn:exit-game>)[= Exit Game]
      ]
    ]

    #place(left + bottom)[
      #set text(size: 18pt)
      #button(lbl: <btn:settings>)[#emoji.gear Settings]
    ]

    // #let player_name = "Nixon"

    // #place(right + top)[
    //   #set text(size: 18pt)

    //   #let size = 60pt
    //   #align(horizon)[
    //     #stack(
    //       dir: ltr,
    //       rect(fill: blue, width: size, height: size),
    //       box(
    //         width: 300pt,
    //         height: size,
    //         fill: base6.transparentize(80%),
    //         inset: 20pt,
    //       )[
    //         #stack(
    //           dir: ltr,
    //           spacing: 1fr,
    //           player_name,
    //           underline[View Profile],
    //         )
    //       ],
    //     )
    //   ]
    // ]
  ]
}

#main_menu(1280, 720)
