#import "../monokai_pro.typ": *
#import "../utils.typ": *

#let main(data, animate, closing, dummy_update) = {
  let set_label(lbl) = {
    if closing {
      label("")
    } else {
      lbl
    }
  }

  box(width: 100%, height: 100%)[
    #let card_contents = (
      (
        <btn:sandbox>,
        base5,
        [
          = Sandbox
          #linebreak()
          Sandbox mode with a mini tutorial contained.
        ],
        dummy_update,
      ),
      (
        <btn:1v1>,
        yellow,
        [
          = 1 v 1
          #linebreak()
          Play against 1 player!
        ],
        dummy_update,
      ),
      (
        <btn:2v2>,
        orange,
        [
          = 2 v 2
          #linebreak()
          Team up and play against 2 players!
        ],
        dummy_update,
      ),
      (
        <btn:3v3>,
        purple,
        [
          = 3 v 3
          #linebreak()
          Team up and play against 3 players!
        ],
        dummy_update,
      ),
    )

    #let section_times = calculate_section_time(
      animate,
      card_contents.len() + 1,
    )

    #place(center + horizon)[
      #set text(size: 1.5em)
      #stack(
        dir: ltr,
        spacing: 2em,
        ..card_contents
          .enumerate()
          .map(it => {
            let (i, (lbl, fill, content, d)) = it
            let time = section_times.at(i)

            let fill = fill.transparentize(100% - 100% * time)
            set text(fill: fill)

            scale(90% + 10% * time)[
              #card_button(
                content,
                lbl: set_label(lbl),
                inters: interactions(),
                fill: fill,
              )]
          }),
      )

      #align(right)[
        #let time = section_times.last()
        #set text(fill: red.transparentize(100% - 100% * time))
        #scale(90% + 10% * time)[
          #button(
            lbl: <btn:cancel-matchmake>,
            inters: interactions(),
            disabled: closing,
          )[
            = Cancel
          ]
        ]
      ]
    ]
  ]
}
