#import "../monokai_pro.typ": *
#import "../utils.typ": *

#let main(is_dead, elapsed_time, remaining_time, percentage, dummy_update) = {
  // Hide the UI when not dead
  if not is_dead {
    return
  }

  // Total respawn time
  let total_time = 5.0
  let pulse_time = calc.sin(elapsed-secs() * 6) * 0.5 + 0.5

  let warning_svg = read("/icons/warning.svg")
  warning_svg = warning_svg.replace(
    "opacity=\"1.0\"",
    "opacity=\"" + str(lerp(1.0 - pulse_time, 0.2, 1.0)) + "\"",
  )

  box(width: 100%, height: 100%)[
    // Dimmed background overlay
    #box(
      width: 100%,
      height: 100%,
      fill: rgb(0, 0, 0, 60%),
    )

    #place(top + center, dy: 6em)[
      #text(
        fill: red.transparentize(70% * pulse_time),
        size: 2em,
      )[= CRITICAL FAILURE #box(image(bytes(warning_svg), height: 1em))]

      // Respawn countdown
      #linebreak()
      #let countdown = calc.clamp(remaining_time, 0.0, total_time)
      #let countdown_str = calc.round(countdown * 10.0) / 10.0

      #text(size: 1.3em, fill: blue)[Respawning in...]\
      #text(size: 1.5em, fill: green)[
        #countdown_str s
      ]

      // Revive progress bar
      #box(
        width: 20em,
        height: 1em,
        fill: base2.transparentize(40%),
      )[
        #box(
          width: 100% * percentage,
          height: 100%,
          fill: blue,
        )
      ]
    ]
  ]
}
