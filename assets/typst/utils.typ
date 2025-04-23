#import "monokai_pro.typ": *

#let lerp(x, low, high) = {
  let diff = high - low
  x * diff + low
}

#let calculate_section_time(
  time,
  total_count,
  section_duration: 0.5,
) = {
  let leftout_duration = 1.0 - section_duration
  let diff = leftout_duration / (total_count - 1)

  let section_times = ()
  for i in range(total_count) {
    let clamped_time = calc.clamp(
      time - i * diff,
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
  disabled: false,
) = {
  context [
    #let time = 0.0

    #if disabled == false {
      for (label, time: t) in inters {
        if label == lbl {
          time = t
        }
      }
    }

    #let fill = if fill != none { fill } else {
      text.fill
    }

    #let raw_svg = read(button_files.at(svg_idx))
    #let raw_svg = (
      raw_svg
        .replace(
          "#fff",
          fill.darken(40% - 10% * time).to-hex(),
        )
        .replace("opacity: .2", "opacity: " + str(lerp(time, 0.5, 0.6)))
    )
    #let background = image(bytes(raw_svg), height: 2.5em)

    #box()[
      #scale(100% + (time * 10%))[
        #background
        #let background_size = measure(background)


        #if time > 0.0 {
          let highlight_fill = fill.transparentize(100%)
          let section_times = calculate_section_time(time, 3)
          for (i, section_t) in section_times.enumerate() {
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

#let settings_button(
  body,
  lbl: label,
  inters: array,
  fill: none,
  svg_idx: 0,
) = {
  context [
    #let fill = if fill != none { fill } else {
      text.fill
    }

    #box(
      width: 2em,
      height: 2em,
      fill: fill,
      stroke: none,
      radius: 50%,
    )[
      #place(center + horizon, body)
    ] #lbl
  ]
}

#let card_button(
  body,
  lbl: label,
  inters: array,
  fill: none,
) = {
  let width = 12em
  let height = 17em
  let time = 0.0

  for (label, time: t) in inters {
    if label == lbl {
      time = t
    }
  }

  let fill = if fill != none { fill } else {
    text.fill
  }

  let raw_svg = read("../icons/card.svg")
  let raw_svg = (
    raw_svg
      .replace(
        "#69cad4",
        fill.darken(40%).to-hex(),
      )
      .replace(
        "fill-opacity: 0.8",
        "fill-opacity: " + str(lerp(time, 0.6, 0.8)),
      )
      .replace(
        "stroke-width: 2px",
        "stroke-width: " + str(lerp(time, 2, 8)) + "px",
      )
  )
  let background = image(bytes(raw_svg), height: height)

  context [

    #box()[
      #scale(100% + (time * 10%))[
        #background
        #let background_size = measure(background)

        #if time > 0.0 {
          let highlight_fill = fill.transparentize(100%)
          let section_times = calculate_section_time(time, 2)
          for (i, section_t) in section_times.enumerate() {
            place(
              center + horizon,
              box(
                width: background_size.width,
                height: background_size.height,
                clip: true,
              )[
                #place(
                  dx: lerp(section_t, 0em, 2em * i) - 6em,
                  rotate(
                    45deg,
                    box(
                      width: 1em,
                      height: 200%,
                      fill: highlight_fill
                        .lighten(60%)
                        .opacify(10% * section_t),
                    ),
                  ),
                )
              ],
            )

            place(
              center + horizon,
              box(
                width: background_size.width,
                height: background_size.height,
                clip: true,
              )[
                #place(
                  dx: lerp(section_t, 0em, -2em * i) + 16em,
                  rotate(
                    45deg,
                    box(
                      width: 1em,
                      height: 200%,
                      fill: highlight_fill
                        .lighten(60%)
                        .opacify(10% * section_t),
                    ),
                  ),
                )
              ],
            )
          }
        }
      ]
      #set text(size: text.size + 0.1em * time)

      #place(
        center + horizon,
        box(
          width: width,
          height: height,
          inset: 2em,
        )[#body],
      )
    ] #lbl
  ]
}

#let parallelogram(
  body,
  height: 1em,
  width: 1em,
  slant: 0.5em,
  fill: base0,
  stroke: base2,
  alignment: center + horizon
) = context {
  let total_width = width + calc.abs(slant)

  box(
    inset: (right: -total_width),
    polygon(
      stroke: stroke,
      fill: fill,
      (0em, 0em),
      (slant, height),
      (width + slant, height),
      (width, 0em),
    ),
  )

  box(
    width: width + calc.abs(slant),
    height: height,
    place(alignment, body),
  )
}
