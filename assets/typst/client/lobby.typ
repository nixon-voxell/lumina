#import "../monokai_pro.typ": *
#import "../utils.typ": *

#let lobby(
  curr_player_count,
  max_player_count,
  room_id,
  dummy_update,
) = {
  box(width: 100%, height: 100%, inset: 2em)[
    #set text(fill: base7)

    #place(center + top)[
      = Waiting for players (#curr_player_count/#max_player_count)
    ]

    #place(bottom + right, dy: 1em)[
      #text(fill: base6)[Room Id: #room_id]
    ]

    #place(top + right)[
      #text(fill: red, size: 0.7em)[
        #button(
          lbl: <btn:exit-lobby>,
          inters: interactions(),
        )[= Exit]
      ]
    ]
  ]
}
