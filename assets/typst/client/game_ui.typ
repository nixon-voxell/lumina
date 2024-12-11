#import "../monokai_pro.typ": *

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
          box(
            image(
              "/icons/bullet-yellow.svg",
              height: 1.3em,
            ),
          ),
          spacing: 5pt,
          // Use the dynamic bullet count from the array
          text(fill: white, 20pt)[*#(bullet_counts.at(x))*],
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
    width: 3em,
    height: 3em,
    fill: base1.transparentize(20%),
    radius: 4pt,
  )[
    #align(horizon)[
      #text(fill: base7)[#team_score]
    ]
  ]
}

#let score_bar(scores, max_score) = {
  set align(horizon)

  let local_percent = 0.1 * 100%
  let other_percent = 100% - local_percent

  let grad_sharpness = 1%
  let local_grad = calc.max(local_percent - grad_sharpness, 0%)
  let other_grad = calc.min(local_percent + grad_sharpness, 100%)

  box(
    width: 50%,
    height: 1em,
    // fill: base7,
    fill: gradient.linear(
      space: rgb,
      (blue.transparentize(70%), 0%),
      (blue.transparentize(70%), local_grad),
      (red.transparentize(70%), other_grad),
      (red.transparentize(70%), 100%),
    ),
    outset: 0.2em
  )[
    #place(left + horizon)[
      #box(
        width: local_percent,
        height: 100%,
        fill: blue,
      )
    ]
    #place(right + horizon)[
      #box(
        width: other_percent,
        height: 100%,
        fill: red,
      )
    ]

    #place(dx: local_percent - grad_sharpness * 0.5)[
      #box(
        width: 0.5em,
        height: 200%,
        fill: base8.transparentize(30%),
      )
    ]
  ]
  // align(horizon)[
  //   #box(width: 850pt)[
  //     #grid(
  //       columns: 3,
  //       column-gutter: 30pt,

  //       // Left bar with score
  //       box(width: 380pt)[
  //         #rect(
  //           width: 100%,
  //           height: 20pt,
  //           inset: 0pt,
  //           fill: white.transparentize(50%),
  //           stroke: (top: 3pt + red),
  //         )[
  //           #place(
  //             left,
  //             rect(
  //               width: (float(scores.at(0)) / float(max_score)) * 100%,
  //               height: 100%,
  //               fill: red,
  //               stroke: (top: 3pt + white),
  //             ),
  //           )
  //         ]
  //       ],

  //       // Center container for score displays
  //       box(width: auto)[
  //         #stack(
  //           dir: ltr,
  //           spacing: 20pt,
  //           score_display(scores.at(0)),
  //           score_display(scores.at(1)),
  //         )
  //       ],

  //       // Right bar with score
  //       box(width: 380pt)[
  //         #rect(
  //           width: 100%,
  //           height: 20pt,
  //           inset: 0pt,
  //           fill: white.transparentize(50%),
  //           stroke: (top: 3pt + blue),
  //         )[
  //           #place(
  //             right,
  //             rect(
  //               width: (float(scores.at(1)) / float(max_score)) * 100%,
  //               height: 100%,
  //               fill: blue,
  //               stroke: (top: 3pt + white),
  //             ),
  //           )
  //         ]
  //       ]
  //     )
  //   ]
  // ]
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
  text(fill: base7, size: 2em)[
    #formatted_minutes:#formatted_seconds
  ]
}

#let main(
  timer,
  weapon_selector,
  score_bar,
) = {
  // text.size
  box(
    width: 100%,
    height: 100%,
    inset: 30pt,
  )[
    #place(center + top)[
      #score_bar
    ]

    #place(top + left)[
      #timer
    ]

    // #place(right + bottom)[
    //   #weapon_selector
    // ]
    // #context measure(box(height: 100%, width: 100%))
  ]
}

