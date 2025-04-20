#import "../utils.typ": *
#import "../monokai_pro.typ": *

#let key(code, pressed) = {
  // Use monospace font so that the size of the box will always
  // be the same if the letter count is the same.
  set text(font: "Consolas", fill: if pressed { base0 } else { base7 })
  let color = if pressed {
    green
  } else {
    base2
  }
  box(
    inset: 0.7em,
    fill: color.lighten(20%).transparentize(20%),
    stroke: color.darken(60%) + 0.2em,
    align(center + horizon, code),
  )
}

#let btn(code, pressed) = {
  // Use monospace font so that the size of the box will always
  // be the same if the letter count is the same.
  set text(font: "Consolas", fill: if pressed { base0 } else { base7 })
  let color = if pressed {
    green
  } else {
    base2
  }
  box(
    inset: 0.7em,
    fill: color.lighten(20%).transparentize(20%),
    stroke: color.darken(60%) + 0.2em,
    radius: 2em,
    align(center + horizon, code),
  )
}

#let stick(lbl, x, y) = {
  let pressed = x != 0.0 or y != 0.0
  set text(font: "Consolas", fill: if pressed { base0 } else { base7 })
  let stick_color = if pressed {
    green
  } else { base2 }

  circle(
    stroke: stick_color.darken(10%).transparentize(50%) + 0.4em,
    width: 7em,
  )[
    #place(
      center + horizon,
      dx: x * 2em,
      dy: -y * 2em,
      circle(
        fill: stick_color.lighten(20%).transparentize(20%),
        stroke: stick_color.darken(30%) + 0.2em,
        inset: 0em,
        width: 5.5em,
      )[#lbl],
    )

  ]
}

#let main(data, dummy_update) = {
  let origin_x = data.origin_x * 1pt
  let origin_y = data.origin_y * 1pt
  set text(size: 1em / data.scale)

  let act = data.act

  box(width: 100%, height: 100%)[
    #place(
      dx: origin_x - 28em,
      dy: origin_y + 20em,
    )[
      #set align(center)
      #if data.is_using_mouse {
        box(inset: (x: 1em))[
          #key([Space], act.dash)

          Dash

          #key([L Shift], act.boost)

          Boost
        ]
        box(inset: (x: 1em))[
          #key([= W], act.w)\
          #box()[
            #key([= A], act.a)
            #key([= S], act.s)
            #key([= D], act.d)
          ]

          Movement
        ]
        box(inset: (x: 1em))[
          #key([= E], act.interact)

          Interact

          #key([= Q], act.ability)

          Ability
        ]
        box(inset: (x: 1em))[
          #key([= R], act.reload)

          Reload

          // #key([= Q], act.ability)

          // Ability
        ]
        box(inset: (x: 1em))[
          Mouse
          #stack(
            dir: ltr,
            spacing: 0.1em,
            rect(
              width: 1em,
              height: 2em,
              fill: if act.attack { green } else { base2 },
              radius: (top-left: 0.3em, bottom-left: 0.3em),
            ),
            rect(
              width: 1em,
              height: 2em,
              fill: base0,
              radius: (top-right: 0.3em, bottom-right: 0.3em),
            ),
          )
          Attack
        ]
      } else {
        box(height: 12em, inset: (y: 1em, left: 1em, right: 2em))[
          #box[
            #key([LT], act.boost) ~ Boost
            #box(width: 6em)
            Attack ~ #key([RT], act.attack)
          ]\
          #box(inset: (x: 1em))[
            #stick([L Stick], act.move_x, act.move_y)

            Movement
          ]
          #box(inset: (x: 1em))[
            #stick([R Stick], act.aim_x, act.aim_y)

            Aim
          ]
        ]
        box(height: 10em)[
          #set align(horizon + center)
          #box(height: 100%)[
            #set align(horizon + right)
            #move(dx: 0.5em)[
              #box([Interact])
              #btn([X], act.interact)
            ]
          ]
          #box(height: 100%)[
            #place(center + horizon, dy: -3.5em)[Reload]
            #btn([Y], act.reload)

            #btn([A], act.dash)
            #place(center + horizon, dy: 3.5em)[Dash]
          ]
          #box(height: 100%)[
            #set align(horizon + right)
            #move(dx: -0.5em)[
              #btn([B], act.ability)
              #box([Ability])
            ]
          ]
        ]
      }
    ]
  ]
}
