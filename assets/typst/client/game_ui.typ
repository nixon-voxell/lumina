#import "../monokai_pro.typ": *

#set page(
  width: auto,
  height: auto,
  margin: 0pt,
)

// Booster meter
#let boostmeter(
  height,
  width,
  red_height,
) = {
  let width = 300pt
  let height = 14pt
  grid(
    columns: (60pt, 1fr),
    rows: (auto),
    gutter: 3pt,
    align(horizon)[
      #stack(
        dir: ltr,
        spacing: 15pt,
        image(
          "/icons/flame-icon.svg",
          height: 2.5em,
        ),
        rect(
          width: width,
          height: height,
          inset: 0pt,
          fill: white,
        )[
          // Add red rectangle as booster overheat signal
          #place(
            top + left,
            rect(
              width: red_height * 100%,
              height: 100% * 100%,
              fill: rgb("EC6F59").saturate(30%),
            ),
          )
        ],
      )
    ],
  )
}

#let weaponselector(
  selected_index,
  num_weapon,
  bullet_counts,
) = {
  let weapon(x) = {
    let color = if x == selected_index {
      white
    } else {
      white.transparentize(70%)
    }
    stack(
      dir: ttb,
      spacing: 12pt,
      rect(
        width: 80pt,
        height: 80pt,
        fill: white.transparentize(85%),
        stroke: 4pt + color,
        radius: 4pt,
      ),
      align(horizon + center)[
        #stack(
          dir: ltr,
          box(image(
            "/icons/bullet-yellow.svg",
            height: 1.3em,
          )),
          spacing: 5pt,
          // Use the dynamic bullet count from the array
          text(fill: white, 20pt)[*#(bullet_counts.at(x))*]
        )
      ],
    )
  }

  let weapons = range(num_weapon).map(weapon)
  box(width: 200pt)[
    #grid(
      columns: (1fr, 1fr),
      ..weapons
    )
  ]
}

#let score_display(team_score) = {
  rect(
    width: 50pt,
    height: 50pt,
    fill: black.lighten(20%),
    radius: 4pt,
  )[
    #align(horizon)[
      #text(fill: white, size: 30pt)[#team_score]
    ]
  ]
}

#let score_bar(scores) = {
  align(horizon)[
    #box(
      width: 850pt,
      height: auto,
      {
        grid(
          columns: 3,
          column-gutter: 30pt,

          // Left bar with score
          box(width: 380pt)[
            #rect(
              width: 100%,
              height: 20pt,
              inset: 0pt,
              fill: white.transparentize(50%),
              stroke: (top: 3pt + red),
            )[
              #place(
                left,
                rect(
                  width: scores.at(0) * 3.8pt,
                  height: 100%,
                  fill: red,
                  stroke: (top: 3pt + white),
                )
              )
            ]
          ],

          // Center container for score displays
          box(width: auto)[
            #stack(
              dir: ltr,
              spacing: 20pt,
              score_display(scores.at(0)),
              score_display(scores.at(1))
            )
          ],

          // Right bar with score
          box(width: 380pt)[
            #rect(
              width: 100%,
              height: 20pt,
              inset: 0pt,
              fill: white.transparentize(50%),
              stroke: (top: 3pt + blue),
            )[
              #place(
                right,
                rect(
                  width: scores.at(1) * 3.8pt,
                  height: 100%,
                  fill: blue,
                  stroke: (top: 3pt + white),
                )
              )
            ]
          ]
        )
      },
    )
  ]
}

#let countdown_timer(total_seconds) = {
  // Ensure non-negative time
  let total = calc.max(total_seconds, 0)

  // Calculate minutes and seconds
  let minutes = calc.floor(total / 60)
  let seconds = calc.floor(calc.rem(total, 60))

  // Format minutes and seconds with leading zeros
  let formatted_minutes = if minutes < 10 {
    "0" + str(minutes)
  } else {
    str(minutes)
  }

  let formatted_seconds = if seconds < 10 {
    "0" + str(seconds)
  } else {
    str(seconds)
  }

  // Display the countdown timer in MM:SS format
  text(fill: base7, size: 45pt)[
    #formatted_minutes:#formatted_seconds
  ]
}

// Input:
// - Length of the entire health bar
// - Max HP
// - Current HP

// Rule
// - Each box represents 10 HP
#let playerhealth(
  current_hp,
  max_hp,
  total_width: 280pt,
  rect_height: 31pt,
  spacing: 8pt,
) = {
  // Calculate the total number of blocks needed
  let blocks = calc.ceil(current_hp / 10)
  let max_blocks = calc.ceil(max_hp / 10)

  // Calculate the width of each block
  let rect_width = (total_width - (max_blocks - 1) * spacing) / max_blocks

  // Define health state colors
  let low_color = red // Red
  let medium_color = orange // Orange
  let medium_high_color = yellow.mix((base7, 50%)) // Bright Yellow
  let healthy_color = green // Green

  // Function to determine block color based on health ratio and block index
  let get_block_color(block_index) = {
    let hp_ratio = current_hp / max_hp

    // Color progression for all blocks
    if block_index < 15 {
      if hp_ratio > 0.7 {
        healthy_color.saturate(80%)
      } else if hp_ratio > 0.5 {
        medium_high_color.saturate(80%)
      } else if hp_ratio > 0.3 {
        medium_color.saturate(80%)
      } else {
        low_color.saturate(80%)
      }
    } else {
      // After 15 blocks, color changes based on overall health
      if hp_ratio > 0.7 {
        healthy_color.saturate(80%)
      } else if hp_ratio > 0.5 {
        medium_high_color.saturate(80%)
      } else if hp_ratio > 0.3 {
        medium_color.saturate(80%)
      } else {
        low_color.saturate(80%)
      }
    }
  }

  grid(
    columns: (60pt, 1fr),
    rows: (auto),
    gutter: 3pt,
    align(horizon)[
      #stack(
        dir: ltr,
        spacing: 15pt,
        rotate(
          -90deg,
          image(
            "/icons/battery.svg",
            height: 3em,
          ),
        ),
        box(fill: white, height: rect_height)[
          // Display blocks for current and max health
          #for i in range(max_blocks) {
            let fill_color = if i < blocks {
              get_block_color(i)
            } else {
              get_block_color(i).transparentize(80%)
            }
            place(dx: i * (rect_width + spacing))[
              #rect(
                width: rect_width,
                height: 100%,
                fill: fill_color,
              )
            ]
          }
        ],
      )
    ],
  )
}

#let main(
  main_width,
  main_height,
  boostmeter,
  timer,
  health,
  weapon_selector,
  score_bar,
) = (
  context {
    let main_width = main_width * 1pt
    let main_height = main_height * 1pt
    box(
      width: 100%,
      height: 100%,
      inset: 50pt,
    )[
      #place(center + top)[
        #score_bar
      ]

      #place(left + bottom)[
        #health
        #boostmeter
      ]

      #place(top + left)[
        #timer
      ]

      // #place(right + bottom)[
      //   #weapon_selector
      // ]
    ]
  }
)

