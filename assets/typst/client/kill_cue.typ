#import "../monokai_pro.typ": *
#import "../utils.typ": *

#let main(name, animate, streak) = context {
  box(width: 100%, height: 100%, inset: 2em)[
    #if animate < 0.99 {
      let (time0, time1) = calculate_section_time(
        animate,
        2,
        section_duration: 0.8,
      )
      let name_len = name.len()
      let str_times = calculate_section_time(time0, name_len)

      place(center + top, dy: 8em)[
        #set text(size: 2em)
        #for (i, t) in str_times.enumerate() {
          box(scale(100% * t, name.at(i)))
        }

        #let name_size = measure([#name])
        #let overshoot = 1em

        #place(left + horizon)[
          #box(
            move(
              dx: -overshoot * 0.5,
              line(
                length: (2 * name_size.width + overshoot) * time1,
                stroke: red.transparentize(30%) + 0.2em,
              ),
            ),
          )
        ]
      ]
    }

    #place(right + bottom, dy: -8em)[
      #text(fill: orange)[#streak x]
    ]
  ]
}
