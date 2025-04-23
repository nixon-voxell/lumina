#import "../monokai_pro.typ": *
#import "../utils.typ": lerp, parallelogram

#let wave(width, height, amplitude, freq, time, fill) = {
  let s_time = (
    calc.sin((time) * freq),
    calc.sin((time + 0.25 * calc.tau) * freq),
    calc.sin((time + 0.5 * calc.tau) * freq),
    calc.sin((time + 0.75 * calc.tau) * freq),
  )

  let points = (
    (0pt, s_time.at(0) * amplitude),
    (width * 0.3333, s_time.at(1) * amplitude),
    (width * 0.6666, s_time.at(2) * amplitude),
    (width, s_time.at(3) * amplitude),
  )

  curve(
    fill: fill.transparentize(50%),
    stroke: fill.lighten(20%) + 0.2em,
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

#let ring(
  width,
  stroke_width,
  fill,
  glow_count: 0,
  glow_thickness: 0.2em,
) = {
  place(
    center + horizon,
    circle(
      width: width,
      height: width,
      stroke: (stroke_width) + fill.desaturate(50%),
    ),
  )
  for i in range(glow_count) {
    let i = i + 1
    place(
      center + horizon,
      circle(
        width: width,
        height: width,
        stroke: (stroke_width + i * glow_thickness)
          + fill.transparentize(i / (glow_count + 1) * 100%),
      ),
    )
  }
}

#let disc_stats(
  width,
  arc_fill,
  wave_fill,
  arc_total_count,
  arc_count,
  wave_amount,
  time,
  arc_spacing: 1deg,
  cap: "butt",
) = {
  // Constants
  let black_transparent = black.transparentize(100%)
  let white_transparent = white.transparentize(100%)
  let arc_percentile = (1 / arc_total_count) - (arc_spacing / 360deg)

  let wave_height = lerp(wave_amount, 0.1, 0.87) * 100%
  let wave_color_mix = calc.pow(wave_amount, 0.2) * 100%
  let wave_color = color.mix(
    (wave_fill, wave_color_mix),
    (red, 100% - wave_color_mix),
  )

  let arc_color_mix = arc_count / arc_total_count * 100%
  let arc_color = color.mix(
    (arc_fill, arc_color_mix),
    (red, 100% - arc_color_mix),
  )
  box(width: width, height: width, radius: width, clip: true)[
    #let offset_time = time + 2.2
    #let water_color = wave_color.darken(40%).saturate(50%)
    #place(
      bottom,
      wave(
        width,
        wave_height + (calc.sin(offset_time + calc.pi * 1.8) * 0.5 + 0.5) * 5%,
        5%,
        1.0,
        offset_time,
        water_color,
      ),
    )
    #place(
      bottom,
      wave(
        width,
        wave_height + (calc.sin(time + calc.pi * 0.2) * 0.5 + 0.5) * 10%,
        10%,
        1.0,
        time,
        water_color,
      ),
    )
    #place[
      #circle(
        width: 100%,
        height: 100%,
        fill: gradient.radial(
          (black_transparent, 0%),
          (water_color.transparentize(80%), 70%),
          (water_color.lighten(80%), 100%),
        ),
      )
    ]
  ]

  let stroke_width = 1.8em
  let distance = stroke_width
  let glow_animation = (calc.sin(time * 2) * 0.5 + 0.5)
  place(top + left)[
    #box(width: width, height: width, radius: width)[
      // Ring background.
      #ring(width + distance, stroke_width, arc_color.darken(80%))
      // Inner ring.
      #ring(
        width + distance - stroke_width,
        0.06em,
        wave_color.mix(arc_color).lighten(40%).transparentize(30%),
        glow_count: 2,
      )
      // Outer ring.
      #ring(
        width + distance + stroke_width,
        0.04em,
        arc_color.lighten(40%).transparentize(30% * glow_animation),
        glow_count: 2,
      )

      // Arc segments.
      #for a in range(arc_count) {
        place(
          center + horizon,
          rotate(
            a * 360deg / arc_total_count + arc_spacing * 0.5,
            arc(
              width + distance,
              stroke_width * 0.5,
              arc_percentile,
              arc_color.darken(30%),
              cap,
            ),
          ),
        )
      }

      // Last arc glow.
      #for i in range(3) {
        let spacing_grow = arc_spacing * 0.3
        let glow_arc_spacing = arc_spacing - (spacing_grow + i * spacing_grow)
        let glow_arc_percentile = (
          (1 / arc_total_count) - (glow_arc_spacing / 360deg)
        )
        place(
          center + horizon,
          rotate(
            (arc_count - 1) * 360deg / arc_total_count + glow_arc_spacing * 0.5,
            arc(
              width + distance,
              stroke_width * (0.6 + i * 0.1),
              glow_arc_percentile,
              arc_color
                .desaturate(20%)
                .transparentize(50% * glow_animation + (i * 20%)),
              cap,
            ),
          ),
        )
      }
    ]]
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
  keybind,
  bg_color: base3.transparentize(30%),
  load_color: base0.transparentize(30%),
  stroke_color: base2.transparentize(30%).lighten(20%),
) = {
  box(
    inset: (x: 0.5em),
    parallelogram(
      height: width,
      width: height,
      slant: -width * 0.6,
      fill: gradient.linear(
        (bg_color, 0%),
        (bg_color, 100% * (1.0 - cooldown)),
        (load_color, 100% * (1.0 - cooldown)),
        (load_color, 100%),
        angle: 90deg,
      ),
      stroke: stroke_color + 0.2em,
    )[#box(inset: (x: height * 0.5))[
        #image(icon_path)
        #place(center + top, dy: -height * 0.6, dx: width * 0.2)[#keybind]
      ]
    ],
  )
}

#let main(data, dummy_update) = {
  if data == none {
    return
  }

  let is_assassin = data.spaceship_type.ends-with("Assassin")
  let ability_icon = if is_assassin { "shadow" } else { "heal" }
  let weapon_icon = if is_assassin { "cannon" } else { "gattling-gun" }

  set align(horizon)
  set rect(inset: 0pt)

  let width = 6em
  let height = 1.3em
  box(width: 100%, height: 100%, inset: 2em)[
    #place(bottom + left)[
      #stack(
        dir: ltr,
        box(
          inset: 1em,
          disc_stats(
            width,
            green.desaturate(20%),
            yellow.desaturate(20%).mix(green.desaturate(20%)),
            calc.ceil(data.max_health),
            calc.ceil(data.health),
            data.boost,
            elapsed-secs() * 3,
          ),
        ),
        box(inset: (left: 2em))[
          #box(
            clip: true,
            image(
              "/icons/" + weapon_icon + ".svg",
              height: width * 0.7,
            ),
          )
          #stack(
            dir: ltr,
            spacing: -0.2em,
            box(
              fill: base3.transparentize(10%),
              inset: 0.4em,
              stroke: base2 + 0.15em,
            )[
              #if data.magazine < 10 { "0" + str(data.magazine) } else {
                data.magazine
              }
            ],
            box(width: 1em),

            ..range(data.magazine_size).map(i => {
              let bullet_icon = if i < data.magazine { "bullet" } else {
                "bullet-used"
              }

              move(
                dy: if i < data.reload_size { 0em } else { 0.7em },
                image("/icons/" + bullet_icon + ".svg", height: 1em),
              )
            }),
          )
        ],
      )
    ]


    #place(bottom + right)[
      #let lumina_percent = 100% * calc.pow(data.lumina_count / 15, 8.0)
      #let sin_lerp_fast = calc.sin(elapsed-secs() * 4.0) * 0.5 + 0.5
      #let sin_lerp = calc.sin(elapsed-secs() * 2.0) * 0.5 + 0.5
      #let cos_lerp = calc.cos(elapsed-secs() * 2.0) * 0.5 + 0.5

      #let lumina_colors = (
        color
          .mix(
            (
              color.mix(
                (blue, 100% * sin_lerp),
                (purple, 100% * (1.0 - sin_lerp)),
              ),
              100% - lumina_percent,
            ),
            (red, lumina_percent),
          )
          .transparentize(30% + (lumina_percent * 30% * sin_lerp_fast)),
        color
          .mix(
            (
              color.mix(
                (blue, 100% * cos_lerp),
                (purple, 100% * (1.0 - cos_lerp)),
              ),
              100% - lumina_percent,
            ),
            (red, lumina_percent),
          )
          .transparentize(30% + (lumina_percent * 30% * sin_lerp_fast)),
      )

      #box(
        stack(
          dir: ltr,
          spacing: 0.3em,
          effect_cooldown_display(
            data.dash_cooldown,
            "/icons/dash.svg",
            height * 2,
            height * 2,
            if data.is_using_mouse { [#text(size: 0.8em)[[Space]]] } else {
              [A]
            },
          ),
          effect_cooldown_display(
            data.ability_cooldown,
            "/icons/" + ability_icon + ".svg",
            height * 3,
            height * 3,
            if data.is_using_mouse { [Q] } else { [B] },
            bg_color: if data.ability_active {
              blue.darken(60% + 20% * sin_lerp_fast)
            } else {
              base3.transparentize(30%)
            },
            stroke_color: if data.ability_active {
              blue.lighten(20% + 20% * sin_lerp_fast)
            } else {
              base3.transparentize(30%).lighten(20%)
            },
          ),
        ),
      )
      #box(inset: (left: -height * 2))[
        #polygon.regular(
          fill: base3.transparentize(60%),
          stroke: (0.1em + 0.1em * sin_lerp_fast)
            + gradient.linear(..lumina_colors.map(col => col.darken(20%))),
          size: height * 2,
          vertices: 3,
        )
        #place(center + horizon, dy: 0.3em)[#data.lumina_count]
        #place(center + bottom, dy: 0.7em)[#text(size: 0.7em)[Lumina]]
      ]
    ]
  ]
}
