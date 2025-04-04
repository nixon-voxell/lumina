#import "monokai_pro.typ": *

#let lerp(x, low, high) = {
  let diff = high - low
  x * diff + low
}

#let calculate_section_time(
  time,
  total_count,
  section_duration: 0.3,
) = {
  let leftout_duration = 1.0 - section_duration
  let diff = leftout_duration / (total_count - 1)

  let section_times = ()
  for i in range(total_count) {
    let clamped_time = calc.clamp(
      time,
      i * diff,
      1.0 - (total_count - i - 1) * diff,
    )

    clamped_time = calc.clamp(
      time - (total_count - i - 1) * diff,
      0.0,
      section_duration,
    )

    section_times.push(clamped_time / section_duration)
  }

  section_times
}

#let button_files = ("../icons/button01.svg", "../icons/button02.svg")

#let button(
  body,
  lbl: label,
  inters: array,
  fill: none,
  svg_idx: 0,
) = {
  context [
    #let time = 0.0

    #for (label, time: t) in inters {
      if label == lbl {
        time = t
      }
    }

    #let fill = if fill != none { fill } else {
      text.fill.transparentize(60%)
    }

    #let raw_svg = read(button_files.at(svg_idx))
    #let raw_svg = raw_svg.replace(
      "#fff",
      fill.lighten(20% * time).opacify(60% * time).to-hex(),
    )
    #let background = image(bytes(raw_svg), height: 2.5em)

    #box()[
      #scale(100% + (time * 10%))[
        #background
        #let background_size = measure(background)


        #if time > 0.0 {
          let highlight_fill = fill.transparentize(100%)
          let section_times = calculate_section_time(time, 3)
          for (i, section_t) in section_times.rev().enumerate() {
            // place(
            //   center + horizon,
            //   box(
            //     width: background_size.width,
            //     height: background_size.height,
            //     outset: (0.1em + i * 0.1em) * section_t,
            //     stroke: (0.2em + i * 0.1em)
            //       + highlight_fill.opacify(5% * section_t),
            //   ),
            // )

            place(
              center + horizon,
              box(
                width: background_size.width,
                height: background_size.height,
                clip: true,
              )[
                #place(
                  dx: lerp(section_t, 2em * i, 2em + 2em * i),
                  rotate(
                    45deg,
                    box(
                      width: 1em,
                      height: 200%,
                      fill: highlight_fill.opacify(10% * section_t),
                    ),
                  ),
                )
              ],
            )
          }
        }
      ]
      #set text(
        size: text.size + 0.1em * time,
        stroke: (
          paint: text.fill.transparentize(100% - 20% * time),
          thickness: 0.15em * time,
        ),
      )

      #place(center + horizon, body)
    ] #lbl
  ]
}

