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
          "../client/game_ui/flame-icon.svg",
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
            "../client/game_ui/bullet-yellow.svg",
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
              stroke: (top: 3pt + white),
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
              stroke: (top: 3pt + white),
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
  let seconds = calc.rem(total, 60)

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
  text(fill: white, size: 45pt)[
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
  rect_width: 21pt,
  rect_height: 31pt,
  spacing: 8pt,
) = {
  // Calculate the ratio of current HP to max HP
  let hp_ratio = current_hp / max_hp

  // Determine the total number of blocks and how many blocks to display based on HP ratio
  let max_blocks = 10 // Adjust this value for more or fewer blocks
  let num_blocks = hp_ratio * max_blocks

  // Define health state colors
  let healthy_color = rgb("A9DC76") // Green
  let medium_color = rgb("FFB454") // Orange
  let low_color = rgb("FF6188") // Red

  // Function to determine block color based on health ratio
  let get_block_color(is_filled) = {
    if not is_filled {
      // For empty blocks, use transparent version of healthy color
      return healthy_color.transparentize(80%)
    }

    // Color thresholds
    if hp_ratio > 0.7 {
      healthy_color.saturate(80%)
    } else if hp_ratio > 0.3 {
      medium_color.saturate(80%)
    } else {
      low_color.saturate(80%)
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
            "../client/game_ui/battery2.svg",
            height: 3em,
          ),
        ),
        box(fill: white, height: rect_height)[
          // Display the fractional blocks based on the HP ratio
          #for i in range(max_blocks) {
            let is_filled = i < num_blocks
            let fill_color = get_block_color(is_filled)
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
      #place(right + horizon)[

      ]

      #place(center + top)[
        #score_bar
      ]

      #place(left + bottom)[
        #health
        #boostmeter
      ]

      #place(top + left)[
        #timer
        }
      ]
      // Directly access the weapon_selector fields without iterating
      #place(center + bottom, dx: 350pt)[
        #weapon_selector
      ]
    ]
  }
)

