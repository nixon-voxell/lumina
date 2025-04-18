#import "../monokai_pro.typ": *
#import "../utils.typ": *

#let main(is_dead, elapsed_time, remaining_time, percentage, dummy_update) = {
  if not is_dead {
    // Hide the UI when not dead
    return 
  }

  // Total respawn time
  let total_time = 5.0

  box(width: 100%, height: 100%)[
    // Dimmed background overlay
    #box(
      width: 100%,
      height: 100%,
      fill: rgb(0, 0, 0, 60%)
    )[]
    #place(top + center, dy: 6em)[
      #box(
        width: 15em,
        height: 15em,
        fill: rgb(0, 0, 0, 80%),
        stroke: 3pt + rgb(255, 50, 50, 50%),
        radius: 8pt,
        inset: 1.2em,
      )[
        #set text(
          font: "IBrand", 
          size: 1em,
          fill: rgb(255, 100, 100),
          weight: "bold",
        )
        #stack(
          dir: ttb,
          spacing: 0.5em,
          text(underline(offset: 3pt, stroke: 1pt + rgb(255, 50, 50))[CRITICAL FAILURE]),
          text(size: 0.8em, fill: rgb(255, 50, 50).desaturate(20%))[
            Spaceship Destroyed
          ],
        )

        // Respawn countdown
        #linebreak()
        #set text(
          size: 1.3em,
          fill: rgb(0, 255, 255),
        )
        #let countdown = calc.clamp(remaining_time, 0.0, total_time)
        #let countdown_str = calc.round(countdown * 10.0) / 10.0
        
        #stack(
          dir: ttb,
          spacing: 0.5em,
          text[RESPAWNING IN],
          text(
            size: 1em,
            weight: "bold",
            fill: rgb(0, 255, 180)
          )[
            #countdown_str ~ s
          ],
        )

        // Revive progress bar
        #place(bottom + center, dy: -0.5em)[
          #box(
            width: 70%,
            height: 5pt,
            fill: rgb(0, 0, 0, 50%),
            stroke: 1pt + rgb(0, 255, 255, 50%),
          )[
            #box(
              width: 100% * percentage,
              height: 100%,
              fill: rgb(0, 255, 255),
            )[]
          ]
        ]
      ]
    ]
  ]
}