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
              dy: if i < data.reload_size or ((i - data.magazine) < data.reload_size){
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
