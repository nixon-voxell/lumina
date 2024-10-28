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

#let weapon_selector(
  width: 60pt,
  height: 60pt,
) = {
  // Place two boxes side by side
  
  box(
    width: width,
    height: height,
    fill: red,
    stroke: blue,
  )
  h(2%)
  box(
    width: width,
    height: height,
    fill: red,
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
#let playerhealth(
  rect_width: 15pt,
  rect_height: 20pt,
  spacing: 7pt,
  num_rectangles: 6
) = {
  text(fill: green, size: 18pt, [= Health])
  box()
  // Stack the green rectangles horizontally
  for i in range(num_rectangles) {
    place(dx: i * (rect_width + spacing))[
      #rect(
        width: rect_width,
        height: rect_height,
        fill: green.saturate(80%),
        // fill: rgb("#00ff00"),
      )
    ]
  }
}


#let main( main_width,
  main_height, boostmeter, timer) = context {
    let main_width = main_width * 1pt
    let main_height = main_height * 1pt
  box(
    width: main_width,
    height: main_height,
    inset: 50pt,
  )[
    #place(right + horizon)[
      #for b in boostmeter{
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
      #playerhealth()
    ]

    #let x = speed_and_bullets()
    #let size_x = measure(x)
    #set text(white, size: 100pt)
    #let half_width = size_x.width / 2


    #place(center + bottom, dx: half_width + 100pt)[
      #weapon_selector()
    ]
  ]
}
