#import "../monokai_pro.typ": *

#let health_display(
  health,
  max_health,
  width,
  height,
) = {
  let spacing = 2pt
  let hp_per_box = 10

  // Calculate the total number of blocks needed
  let box_num = calc.ceil(health / hp_per_box)
  let total_box_num = calc.ceil(max_health / hp_per_box)

  if total_box_num == 0 {
    return
  }

  // Calculate the width of each block
  let rect_width = (width - (total_box_num - 1) * spacing) / total_box_num

  // Define health state colors
  let low_color = red
  let medium_color = orange
  let medium_high_color = yellow.mix((base7, 50%))
  let healthy_color = green

  // Function to determine block color based on health ratio and block index
  let box_color = {
    let health_ratio = health / max_health

    if health_ratio > 0.7 {
      healthy_color
    } else if health_ratio > 0.5 {
      medium_high_color
    } else if health_ratio > 0.3 {
      medium_color
    } else {
      low_color
    }
  }

  return box(fill: none, height: height)[
    // Display blocks for current and max health
    #for i in range(total_box_num) {
      let fill_color = if i < box_num {
        box_color
      } else {
        base7.transparentize(80%)
      }
      place(dx: i * (rect_width + spacing))[
        #rect(
          width: rect_width,
          height: 100%,
          fill: fill_color,
        )
      ]
    }
  ]
}

#let dash_cooldown_display(
  cooldown,
  icon_path,
  width,
  height,
) = {
  let fill_height = cooldown * 100%
  let border_radius = 0.4em

  box(
    width: width,
    height: height,
    fill: base7.transparentize(90%),
    radius: border_radius,
    stroke: (paint: base7.transparentize(50%), thickness: 0.1em),
    clip: true,
  )[
    // Icon and cooldown overlay
    #box(inset: 0.3em, image(icon_path))

    // Dark overlay inside the icon filling from bottom to top
    #place(bottom)[
      #rect(
        width: 100%,
        height: fill_height,
        fill: black.transparentize(10%),
      )
    ]
  ]
}

#let main(data) = {
  if data == none {
    return
  }

  set align(horizon)
  set rect(inset: 0pt)

  let width = 17em
  let height = 1.3em
  box(width: 100%, height: 100%, inset: 2em)[
    #place(bottom + left)[
      #grid(
        columns: (auto, 1fr),
        column-gutter: 0.8em,
        row-gutter: 1em,
        image(
          "/icons/health.svg",
          height: 1.5em,
        ),
        health_display(data.health, data.max_health, width, height),

        image(
          "/icons/electric-refueling.svg",
          height: 1.5em,
        ),
        rect(
          width: width,
          height: height,
          fill: base1,
          stroke: orange.lighten(40%).transparentize(80%) + 2pt,
        )[
          // Add red rectangle as booster overheat signal
          #place(
            rect(
              width: data.boost * 100%,
              height: 100%,
              fill: gradient.linear(orange.darken(30%), orange),
            ),
          )
        ],
      )
    ]

    #place(bottom + right)[
      #dash_cooldown_display(
        data.dash_cooldown,
        "/icons/dash.svg",
        height * 2,
        height * 2,
      )
    ]
  ]
}
