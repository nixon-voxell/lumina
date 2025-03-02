#import "../monokai_pro.typ": *
#import "../utils.typ": *

#let lobby(
  hovered_button,
  hovered_animation,
  curr_player_count,
  max_player_count,
  room_id,
) = {
  interactable_window(
    hovered_button: hovered_button,
    hovered_animation: hovered_animation,
  )[
    #box(width: 100%, height: 100%, inset: 2em)[
      #set text(fill: base7)

      #place(center + top)[
        = Waiting for players (#curr_player_count/#max_player_count)
      ]

      #place(bottom + right)[
        Room Id: #room_id
      ]

      #place(top + right)[
        #text(fill: red)[#button(lbl: <btn:exit-lobby>)[= Exit Lobby]]
      ]
    ]
  ]
}
