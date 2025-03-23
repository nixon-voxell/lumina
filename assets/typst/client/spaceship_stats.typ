#import "../monokai_pro.typ": *

#let wave(width, height, amplitude, freq, time, fill, offset: 0) = {
  let s_time = (
    calc.sin((time) * freq + offset),
    calc.sin((time + 0.25 * calc.tau) * freq + offset),
    calc.sin((time + 0.5 * calc.tau) * freq + offset),
    calc.sin((time + 0.75 * calc.tau) * freq + offset),
  )

  let points = (
    (0pt, s_time.at(0) * amplitude),
    (width * 0.3333, s_time.at(1) * amplitude),
    (width * 0.6666, s_time.at(2) * amplitude),
    (width, s_time.at(3) * amplitude),
  )

  curve(
    fill: fill,
    curve.move(points.at(0)),
    curve.cubic(points.at(1), points.at(2), points.at(3)),
    curve.line((width, height)),
    curve.line((0pt, height)),
    curve.line(points.at(0)),
  )
}

#let arc(width, thickness, arc_percentile, paint, cap) = {
  circle(
    width: width,
    stroke: (
      paint: paint,
      thickness: thickness,
      cap: cap,
      dash: (
        calc.pi * width * arc_percentile,
        // Add 10 for potential math error's sake.
        width * calc.pi + 10pt,
      ),
    ),
  )
}

#let disc_stats(
  width,
  fill,
  arc_total_count,
  arc_count,
  wave_time,
  arc_spacing: 2deg,
  cap: "butt",
) = {
  // Constants
  let black_transparent = black.transparentize(100%)
  let white_transparent = white.transparentize(100%)
  let height = width
  let arc_percentile = (1 / arc_total_count) - (arc_spacing / 360deg)

  box(width: width, height: height)[
    #place(
      center + horizon,
      box(
        width: width,
        height: height,
        radius: width,
        clip: true,
      )[
        #place(
          bottom,
          wave(
            width,
            96pt,
            10pt,
            0.7,
            wave_time,
            fill.darken(50%).transparentize(50%),
          ),
        )
        #place(
          bottom,
          wave(
            width,
            100pt,
            15pt,
            1.0,
            wave_time,
            fill.darken(40%).saturate(50%).transparentize(50%),
          ),
        )
        #place[
          #circle(
            width: 200pt,
            height: 200pt,
            fill: gradient.radial(
              (black_transparent, 0%),
              (fill.transparentize(100%), 80%),
              (fill.desaturate(80%).transparentize(50%), 100%),
              // (orange.desaturate(10%).transparentize(91%), 80%),
              // (orange.desaturate(10%).transparentize(97%), 100%),
              // (transparent, 100%)
            ),
          )
        ]
      ],
    )

    #let stroke_width = 40pt
    #let distance = stroke_width
    // Ring background.
    #place(
      center + horizon,
      circle(
        width: width + distance,
        height: height + distance,
        stroke: stroke_width + fill.darken(80%),
      ),
    )
    // Inner ring.
    #place(
      center + horizon,
      circle(
        width: width + distance - stroke_width,
        height: height + distance - stroke_width,
        stroke: 3pt + fill.desaturate(50%),
      ),
    )
    // Inner ring glow.
    #place(
      center + horizon,
      circle(
        width: width + distance - stroke_width,
        height: height + distance - stroke_width,
        stroke: 6pt + fill.desaturate(50%).transparentize(50%),
      ),
    )
    #place(
      center + horizon,
      circle(
        width: width + distance - stroke_width,
        height: height + distance - stroke_width,
        stroke: 9pt + fill.desaturate(50%).transparentize(80%),
      ),
    )

    // Outer ring.
    #place(
      center + horizon,
      circle(
        width: width + distance + stroke_width,
        height: height + distance + stroke_width,
        stroke: 3pt + fill.desaturate(50%),
      ),
    )
    // Outer ring glow.
    #place(
      center + horizon,
      circle(
        width: width + distance + stroke_width,
        height: height + distance + stroke_width,
        stroke: 6pt + fill.desaturate(50%).transparentize(50%),
      ),
    )
    #place(
      center + horizon,
      circle(
        width: width + distance + stroke_width,
        height: height + distance + stroke_width,
        stroke: 9pt + fill.desaturate(50%).transparentize(80%),
      ),
    )

    #for a in range(arc_count) {
      place(
        center + horizon,
        rotate(
          a * 360deg / arc_total_count,
          arc(
            width + distance,
            stroke_width * 0.5,
            arc_percentile,
            fill.darken(30%),
            cap,
          ),
        ),
      )
    }

    #place(
      center + horizon,
      rotate(
        (arc_count - 1) * 360deg / arc_total_count,
        arc(
          width + distance,
          stroke_width * 0.6,
          arc_percentile,
          fill.transparentize(50%),
          cap,
        ),
      ),
    )
    #place(
      center + horizon,
      rotate(
        (arc_count - 1) * 360deg / arc_total_count,
        arc(
          width + distance,
          stroke_width * 0.7,
          arc_percentile,
          fill.transparentize(80%),
          cap,
        ),
      ),
    )
    #place(
      center + horizon,
      rotate(
        (arc_count - 1) * 360deg / arc_total_count,
        arc(
          width + distance,
          stroke_width * 0.8,
          arc_percentile,
          fill.transparentize(90%),
          cap,
        ),
      ),
    )
  ]
}

#let health_display(
  health,
  max_health,
  width,
  height,
) = {
  let spacing = 0.2em
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

  return box(
    fill: black,
    outset: 0.3em,
    radius: 0.3em,
    height: height,
    width: width,
  )[
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
          // radius: 0.1em,
        )
      ]
    }
  ]
}

#let effect_cooldown_display(
  cooldown,
  icon_path,
  width,
  height,
  bg_color: base4,
) = {
  let fill_height = cooldown * 100%
  let border_radius = 0.4em

  box(
    width: width,
    height: height,
    fill: bg_color.transparentize(20%),
    radius: border_radius,
    stroke: (paint: bg_color.lighten(40%), thickness: 0.1em),
    clip: true,
  )[
    // Icon and cooldown overlay
    #box(inset: 0.5em, image(icon_path))

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

  let is_assassin = data.spaceship_type.ends-with("Assassin")
  let ability_icon = if is_assassin { "shadow" } else { "heal" }
  let weapon_icon = if is_assassin { "cannon" } else { "gattling-gun" }

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
          radius: 0.2em,
        )[
          // Add red rectangle as booster overheat signal
          #place(
            rect(
              width: data.boost * 100%,
              height: 100%,
              fill: gradient.linear(orange.darken(30%), orange),
              radius: 0.2em,
            ),
          )
        ],

        box(width: 1.5em, height: 1.5em, clip: true)[
          #image(
            "/icons/" + weapon_icon + ".svg",
            height: 1.5em,
          )
        ],
        stack(
          dir: ltr,
          spacing: -0.2em,
          box(
            fill: base6.transparentize(70%),
            inset: 0.2em,
            radius: 0.2em,
            stroke: base5,
          )[
            #if data.magazine < 10 {
              "0" + str(data.magazine)
            } else {
              data.magazine
            }
          ],
          box(width: 1em),

          ..range(data.magazine_size).map(i => {
            let bullet_icon = if i < data.magazine { "bullet" } else {
              "bullet-used"
            }

            move(
              dy: if i < data.reload_size {
                0em
              } else {
                0.7em
              },
              image(
                "/icons/" + bullet_icon + ".svg",
                height: 1em,
              ),
            )
          }),
        ),
      )
    ]

    #place(bottom + right)[
      #stack(
        dir: ltr,
        spacing: 1em,
        effect_cooldown_display(
          data.dash_cooldown,
          "/icons/dash.svg",
          height * 2,
          height * 2,
        ),
        effect_cooldown_display(
          data.ability_cooldown,
          "/icons/" + ability_icon + ".svg",
          height * 3,
          height * 3,
          bg_color: if data.ability_active { blue.darken(60%) } else { base4 },
        ),
      )
    ]
  ]
}
