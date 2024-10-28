#import "monokai_pro.typ": *

#set page(
  width: auto,
  height: auto,
  fill: black,
  margin: 0pt,
)


// Booster meter

#let boostmeter(
  height,
  width,
  red_height
) = {

  let width = 50pt
  let height = 400pt
  rect(
    width: width,
    height: height,
    inset: 0pt,
    fill: white
  )[
    // Add red rectangle as booster overheat signal
     #place(
   center + bottom,
  rect(
    width: width,
    height: red_height * 100%,
    fill: red
  ))
  ] 
}

#let weaponselector(
  width,
  height,
  box1,
  box2,
) = {
  let width = 60pt
  let height = 60pt

  box(
    width: width,
    height: height,
    fill: red,
    stroke: if box1 { blue } else { none }, // Apply stroke based on state
  )
  h(2%)
  box(
    width: width,
    height: height,
    fill: red,
    stroke: if box2 { blue } else { none }, // Apply stroke based on state
  )
}


#let speed_and_bullets(
  width: 400pt,
  height: 80pt,
) = {
  box(
    width: width,
    height: height,
    fill: white
  )[
    #place(left + horizon)[
      #text(fill: black, 15pt)[= Speed: 70km/h] 
    ]

    #place(right + horizon)[
      #text(fill:black, 15pt)[= Bullets: 30]
    ]
  ]

  
}

#let shop(
  width: 100pt,
  height: 80pt,
  x_offset: 0pt,
  y_offset: 0pt
) = {
  rect(
    width: width,
    height: height,
    fill: yellow
  )
}

#let timer() ={
    text(fill: white, 45pt)[= 00:00]
  
}

#let objectives(
  radius: 140pt,
  
) = {
  circle(radius: radius, fill: yellow)
  
  place(center + horizon)[
    #text(fill: black, 25pt)[ = Objectives]
  ]
}

#let countdown_timer(minutes, seconds) = {
    text(fill: white, size: 45pt)[#minutes:#seconds]
}

// Input:
// - Length of the entire health bar
// - Max HP
// - Current HP

// Rule
// - Each box represents 10 HP
#let playerhealth(current_hp, max_hp, rect_width: 15pt, rect_height: 20pt, spacing: 7pt) = {
    // Calculate the ratio of current HP to max HP
    let hp_ratio = current_hp / max_hp

    // Determine the total number of blocks and how many blocks to display based on HP ratio
    let max_blocks = 10  // Adjust this value for more or fewer blocks
    let num_blocks = hp_ratio * max_blocks

    text(fill: green, size: 18pt)[= Health]

    box()[
        // Display the fractional blocks based on the HP ratio
        #for i in range(max_blocks) {
            let fill_color = if(i < num_blocks) { green.saturate(80%) } else { green.transparentize(80%) }
            place(dx: i * (rect_width + spacing))[
                #rect(
                    width: rect_width,
                    height: rect_height,
                    fill: fill_color,
                )
            ]
        }
    ]
}


#let main(main_width, main_height, boostmeter, timer,health, weapon_selector) = context {
    let main_width = main_width * 1pt
    let main_height = main_height * 1pt
    box(
        width: main_width,
        height: main_height,
        inset: 50pt,
    )[ 
        #place(right + horizon)[
            #for b in boostmeter {
                b
            }
        ]
      
        #place(bottom + center)[
            #speed_and_bullets()
        ]

        #place(top + center)[
            #for t in timer {
                t
            }
        ]
      
        #place(left + bottom)[
            #objectives()
        ]
      
        #place(left + top)[
          #for h in health {
             h
      }
    ]

        // Directly access the weapon_selector fields without iterating
        #place(center + bottom, dx: 100pt)[
            #weaponselector(60pt, 60pt, false, true)
    ]
    ]
}

