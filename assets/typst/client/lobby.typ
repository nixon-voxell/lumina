#import "../monokai_pro.typ": *
#import "../utils.typ": *

#set page(
  height: auto,
  width: auto,
  fill: base0,
)

#let lobby(
  width,
  height,
  curr_player_count: 0,
  max_player_count: 0,
  room_id: none,
  hovered_button: "",
  animate: 0.0,
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
    #place(center + top)[
      #set text(size: 24pt)
      = Waiting for players (#curr_player_count/#max_player_count)
    ]

    #place(bottom + right)[
      #set text(size: 12pt)

      Room Id: #room_id
    ]

    #place(bottom + left)[
      #set text(size: 14pt)
      #text(fill: red)[#button(lbl: <btn:exit-lobby>)[= Exit Lobby]]
    ]
  ]
}
