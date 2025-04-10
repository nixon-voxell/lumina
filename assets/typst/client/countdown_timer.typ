#import "../monokai_pro.typ": *

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

  box(width: 100%, height: 100%, inset: 2em)[
    #place(top + left)[
      #box()[
        #image("../../icons/button02.svg", height: 3em)
        #place(center + horizon)[
          // Display the countdown timer in MM:SS format
          #text(fill: base7, size: 2em)[
            #formatted_minutes:#formatted_seconds
          ]
        ]
      ]
    ]
  ]
}
