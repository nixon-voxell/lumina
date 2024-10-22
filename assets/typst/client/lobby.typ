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
    #box(width: 100%, height: 100%, inset: (x: 4.6%, y: 8%))[
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
  ]
}
