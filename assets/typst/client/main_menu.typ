#import "../monokai_pro.typ": *
#import "../utils.typ": *

#set page(
  height: auto,
  width: auto,
  fill: base0,
)

#let connect_server() = {
  set text(size: 24pt)

  place(center + horizon)[
    #set text(fill: base7)
    = Connecting to server...

    #linebreak()

    #stack(dir: ltr)[
      #text(fill: yellow)[#button(lbl: <btn:reconnect>)[Reconnect]]
      #text(fill: red)[#button(lbl: <btn:exit-game>)[Exit Game]]
    ]
  ]
}

#let test_x = none

#let main_menu(
  width,
  height,
  hovered_button: "",
  animate: 0.0,
  connected: false,
) = {
  set text(fill: base7)


  show label(hovered_button): body => [
    #let box_fill = text.fill.transparentize(((1.0 - animate) * 100%))
    #set text(
      fill: color.mix(
        (text.fill, ((1.0 - animate) * 100%)),
        (base0, animate * 100%),
      ),
    )
    #box(fill: box_fill, radius: 10pt, outset: (animate * 6pt))[#body]
  ]

  let width = (width * 1pt)
  let height = (height * 1pt)

  box(
    width: width,
    height: height,
    inset: (x: width * 6%, y: height * 6%),
  )[
    #if connected == false {
      connect_server()
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
