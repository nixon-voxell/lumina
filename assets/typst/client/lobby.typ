#import "../monokai_pro.typ": *
#import "../utils.typ": *

#let lobby(
  hovered_button: none,
  hovered_animation: 0.0,
  curr_player_count: 0,
  max_player_count: 0,
  room_id: none,
) = {
  interactable_window(
    hovered_button: hovered_button,
    hovered_animation: hovered_animation,
  )[
    #set text(fill: base7)

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
