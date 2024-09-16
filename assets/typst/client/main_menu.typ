#import "../monokai_pro.typ": *

#set page(
  height: auto,
  width: auto,
  fill: base0,
)

#let button(body, lbl: label) = {
  [#box(inset: 16pt)[#body] #lbl]
}

#let main_menu(
  width,
  height,
  btn_highlight: "",
  animate: 0.0,
) = {
  set text(fill: base8)
  show label(btn_highlight): body => [
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
    #place(left + horizon)[
      #set text(size: 48pt)
      #text(fill: yellow)[= Lumina]

      #linebreak()

      #move(dx: 2%, dy: -50pt)[
        #set text(size: 28pt, fill: base7)
        #text(fill: green)[#button(lbl: <btn:play>)[= Play]]\
        #text(fill: purple)[#button(lbl: <btn:luminators>)[= Luminators]]\
        #button(lbl: <btn:tutorial>)[= Tutorial]\

        #set text(size: 16pt, fill: red.transparentize(40%))
        #button(lbl: <btn:exit-game>)[= Exit Game]
      ]
    ]

    #place(left + bottom)[
      #set text(size: 18pt)
      #emoji.gear Settings
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
