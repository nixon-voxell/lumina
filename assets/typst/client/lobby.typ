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
) = {
  set text(fill: base7)

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
  ]
}
