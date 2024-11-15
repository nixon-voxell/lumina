// Talk about the approaches that we will take on making the game rather than the details of the game.
// Discuss about stuff that will influence the gameplay.
// Always start with the general items and then move to the specifics.
// Be careful when using words like (should, could, will). Be precise on the degree of promise that you are putting in the technical document.
// Treat it as a working document and leave it open to change.

#import "monokai_pro.typ": *

#set page(fill: base0)
#set text(size: 11pt, fill: base7)
#set table(stroke: base6)
#set heading(numbering: "1.")

#show link: set text(style: "italic", fill: base8.mix(blue))
#show outline.entry.where(level: 1): it => {
  v(14pt, weak: true)
  strong(it)
}
#show heading: it => {
  if it.level > 2 {
    emph(it.body)
  } else {
    it
  }
}

#let lumina = [*Lumina*]
#let luminara = [*Luminara*]
#let tesseract = [*Tesseract*]
#let dauntless = [*Dauntless*]

#let info(body) = {
  text(fill: blue, body)
}
#let warn(body) = {
  text(fill: yellow, body)
}
#let danger(body) = {
  text(fill: red, body)
}
#let game_ref(body) = {
  text(fill: base8.mix(green), emph(body))
}

#align(center)[
  #text(size: 24pt)[*Lumina*]\
  _Game Design Document (GDD)_
]

= Quick Summary

#underline[*#lumina is a 2D top down fast paced objective based PvPvE game.*]
Players in a team of 3 will be tasked to control a spaceship to fight mobs, collect motes, and gain domninance over the opposing team in a fully procedurally-generated world.
This game blends the idea from #game_ref[Destiny 2's Gambit] game mode and fast paced top down games like #game_ref[Astro Duel 2], #game_ref[Intravenous 2], and #game_ref[Ruiner].

== Game Inspirations

#[
  #set image(width: 80%)
  #set text(size: 10pt)

  #link("https://www.youtube.com/watch?v=TSBdw668c4I")[
    #figure(
      image("inspirations/destiny 2 gambit.jpg"),
      caption: "Destiny 2 gambit mode",
    )
  ]

  #link("https://www.youtube.com/watch?v=FhqtwLV4iX8")[
    #figure(image("inspirations/astro duel 2.jpg"), caption: "Astro Duel 2")
  ]

  #link("https://youtu.be/-oTUZz_BGy4?si=E-nqoIJl26iZyxPC&t=78")[
    #figure(image("inspirations/ruiner.jpg"), caption: "Ruiner")
  ]

  #link("https://www.youtube.com/watch?v=dlVUqgFn4Ew")[
    #figure(image("inspirations/intravenous 2.jpg"), caption: "Intravenous 2")
  ]

  #link("https://www.youtube.com/watch?v=dH6hCAK24Ok")[
    #figure(image("inspirations/ape out.jpg"), caption: "Ape Out")
  ]

  #link("https://www.youtube.com/watch?v=puhw1bEVtro")[
    #figure(image("inspirations/neon chrome.png"), caption: "Neon Chrome")
  ]
]


#pagebreak()

#outline()

#pagebreak()

= Design Pillars

We use design pillars to focus design choices as we move through the project.

#link("https://www.gamedeveloper.com/design/fourteen-forms-of-fun")[Types of fun or enjoyment] which are key to the user experience:

#table(
  columns: (1fr, 1fr),
  [*Competition*], [*Power*],
  [_An activity where the goal is to show one's superiority._],
  [_Capacity of having a strong effect, of acting with strength._],

  [
    Players will feel the urge to gain an advantage over their opponents.
    They must compete with each other on limited resources and avoid death penalties.
  ],
  [
    As players gain more #lumina, they can use it to their advantage by combo depositing to receive temporary buffs or purchasing better weapons.
  ],
)


= Audience & Market

// Figure out your audience.
// If you don't know your audience, start by excluding audience who will not play your game.

This game will target gamers who loves fast paced multipalyer games like #game_ref[Apex Legends] and #game_ref[Astro Duel 2].
It will particularly appeal to gamers who love the mix of competitive PvP and PvE like #game_ref[Destiny 2's Gambit] game mode and #game_ref[World War Z].

#table(
  columns: (auto, auto),
  inset: 8pt,
  [*Platform*], [PC Native (Windows/Mac/Linux)],
  [*Genre*], [2D, Co-op, PvP, PvE, Top-down],
)

#pagebreak()

= Core Gameplay

// What's actually happening when the player plays this game?
// Set out a MVP version of the base game.
// Playtest your game early on.
// Learn how to justify something, come up with reasons why xxx.
// Say as much as you can in as few words as possible.

This is the core gameplay loop and mechanics that the game will have.
Each mechanic should positively impact the player experience towards our design pillars.

#figure(image("game loop.png", width: 90%), caption: "Game Loop")

As a player, your goal is to gain dominance of the #tesseract.
This is achieved by moving the effect meter towards the opposite side.

== Game Loop

+ Start the game with a team of 3.
+ #danger[Eliminate] #dauntless (mobs) to obtain #lumina (currency).
+ Players can use #lumina to:
  - Feed it into the #tesseract (depositor) to #info[increase your team's dominance].
  #align(center)[_or..._]
  - #info[Purchase better equipments] (weapons for this prototype) from the shop.
+ Team with #warn[total dominance] *win!*
+ If timer runs out (approx. 15mins) and no team manages to gain total dominance, the team with #warn[most dominance] wins (this is subject to change into something like a sudden death post prototype phase).

== Game Mechanics

=== Controls

#table(
  columns: (auto, 1fr, 1fr),
  table.header(
    [*Control*],
    [*PC*],
    [*Console*],
  ),

  [Move], [WASD], [Left Stick],
  [Brake], [Space], [L],
  [Boost], [Right Mouse], [L2],
  [Interact], [E], [South],
  [Attack], [Left Mouse], [R2],
  [Aim], [Mouse Cursor], [Right Stick],
)

=== Player

#table(
  columns: (auto, 1fr),
  [*Spaceship*],
  [
    - Physics simulated.
    - Thrusters only pushes spaceships from behind.
    - Direction controlled by _Move_ controls.
  ],

  [*Weapon*],
  [
    - Direction of weapon will snap to _Aim_ controls.
    - Apart from the default weapon, each weapon when purchased will have a limited amount of ammos.
    - All weapons will have a magazine size (reload to replenish).
    - Weapons can be used to attack mobs and opponents.
    #table(
      columns: (auto, 1fr),
      table.header([*Weapon*], [*Characteristics*]),
      [Cannon (default)], [moderate firing rate, moderate damage],
      [Gattling gun], [high firing rate, large mag],
      [Missle], [slow firing rate, area damage, no honing],
    )
    #text(size: 0.8em)[_Types of weapon, non-exhaustive, but good amount for the prototype._]
  ],
)

=== Death

When players get eliminated by #info[mobs / opponents]:
- Get a #danger[death penalty] (This includes dropping all your #lumina, a time delay before respawn, and dropping your purchased weapon).
- Respawn at spawn location with a 5 secs immunity.

=== Combo Deposition

The combo deposition is meant to reward players who takes risks to gather large amount of #lumina and deposit them in one go.
(+ive feedback loop)

#table(
  columns: (auto, 1fr),
  table.header([*Risk*], [*Reward*]),
  [10 #lumina], [30 seconds of alternate dimension time.],
  [20 #lumina],
  [40 seconds of alternate dimension time + 5 #lumina chain reaction.],

  [30 #lumina],
  [60 seconds of alternate dimension time + 7 #lumina chain reaction.],
)

#text(size: 0.8em)[_Alternate dimension is when players are given the ability to see enemies as light instead of darkness. (See @light-vs-dark)_]

#pagebreak()

=== Environment (Light vs Dark) <light-vs-dark>

#figure(image("light vs dark.png", width: 90%), caption: "Light vs Dark")

The default environment background will be completely dark.
#info[Ally] spaceships will help #info[*_illuminate_*] the scene while #danger[enemy] spaceships will #danger[*_consume_*] light.
Some important props around the world will also illuminate the scene (e.g. #tesseract, #lumina).
While the other normal props and obstacles will just block lights.

// #pagebreak()

// = Gameplay Balance & Pacing

// Balancing in multiplayer: make sure players don't feel that they get bullied by other players.

#pagebreak()

= Visual Style & Aesthetics

// Color palette(?)
// Reference visual styles, movies, games, etc.

#let palette_box(cols, darken) = {
  for col in cols {
    box(fill: col.darken(darken), width: 40pt, height: 20pt)
    h(0.65em)
  }
}

#align(center)[
  #palette_box((purple, blue, green, yellow, orange, red), 80%)\
  #palette_box((purple, blue, green, yellow, orange, red), 50%)\
  #palette_box((purple, blue, green, yellow, orange, red), 0%)\
]

#grid(
  columns: (1fr, 1fr),
  column-gutter: 8pt,
  row-gutter: 8pt,
  image("visual style/minions assemble.png"),
  image("visual style/minions tunnel.png"),

  image("visual style/evil minion.png"),
  image("visual style/el macho lair.jpg"),

  image("visual style/light cycle ally.png"),
  image("visual style/light cycle enemy.png"),

  image("visual style/uprising enemy.png"),
  image("visual style/bullet echo.png"),
)

#pagebreak()

== Environment Design

The environment design should be based on the void.
The void is the remains of a past civilization or intelligent being that was destroyed by #dauntless who thrives on #lumina.

#grid(
  columns: (1fr, 1fr),
  column-gutter: 8pt,
  row-gutter: 8pt,
  image("environment/dreaming city.png"),
  image("environment/dreaming city future.png"),

  image("environment/stormy destiny2.png"),
  image("environment/stormy1 destiny2.png"),
)

It will contain:

- Dark space with structures floating around.
- Destroyable small metal structures detached from the main boundary.
- Portal for teleporting.

#pagebreak()

== Spaceship Design

// How are they presented in the game?
// How do players differentiate other players in the game? (colors? shapes?)
// Why should they be the way they are? (justify)

Aim for #info[futuristic, modern, and clean] design.
Spaceships will be view from the above (top down), and it won't be filling much screen space.
Make it simple and easy to recognize without too much details.

#grid(
  columns: (1fr, 1fr),
  column-gutter: 8pt,
  row-gutter: 8pt,
  image("spaceship/large light jet full.png"),
  image("spaceship/small light jet.png"),

  image("spaceship/destiny 2 spaceship.png"),
  image("spaceship/2d spaceship pack.png"),

  image("spaceship/2d spaceship pack2.jpg"),
)

#pagebreak()

== Character Design

Characters are aimed to represent the form of a light bulb / luminance source.
The idea is to merge the art aesthetic of #game_ref[Tron] like feel into the mischievous, playful world of #game_ref[minions].

#align(center)[
  #stack(
    dir: ltr,
    spacing: 10pt,
    image("character/quorra.png", width: 40%),
    align(horizon)[#text(size: 2em)[$+$]],
    image("character/minion lightbulb.png", width: 40%),
  )

  #text(size: 2em)[$arrow.b$]
  #stack(
    dir: ltr,
    spacing: 20pt,
    box(width: 30pt),
    image("character/minion evil edit.png", width: 30%),
    align(horizon)[#text(size: 2em)[... ?]],
  )
]

#pagebreak()

= Setting

Set on #luminara, a planet that is currently in the middle of an energy crisis war.
The inhabitants of the planet (_Luminites_ and _Luminids_) ventures into the void to fight for resources (#lumina).

#table(
  columns: (auto, 1fr),
  [*Luminites*],
  [Luminarian who wants to exploit #lumina in hope of a technology breakthrough which will make them independant of #lumina.],

  [*Luminids*],
  [Luminarian who wants to conserve #lumina and save the planet (they think the technology breakthrough is infeasible).],
)

== Tone

The world of #luminara strikes a balance between whimsical and serious, blending playful mischief with an undertone of cunning ambition.
While the characters, _Luminites_ and _Luminids_, have a lighthearted, almost impish charm, their actions are driven by a calculated desire to outwit and outperform each other.
This creates an atmosphere reminiscent of _"Despicable Me"_, filled with sly humor and mischievous antics, but layered with stakes that emphasize clever strategies and resourceful thinking.

== Character Traits & Motivations

Even though the _Luminites_ and _Luminids_ are in a war, they were actually once the same species, living in harmony.
Not all _Luminites_ and _Luminids_ harbor genuine hatred for one another, many are driven to battle by the ambitions and orders of their leaders and politicians..
This struggle mirrors our own world, where chaos and conflict are often incited by those in power.
The never-ending war rages on, even as their home planet, #luminara, cries out in agony...

// = Business Model

#pagebreak()

= Misc

- *Tech Stack*:
  #table(
    columns: (auto, auto),
    inset: 6pt,
    // align: horizon,
    table.header(
      [*Type*],
      [*Solution*],
    ),

    [Game Engine], [Bevy],
    [Level design], [Blender + Blenvy],
    [Vector graphics], [Velyst (Vello + Typst)],
    [Multiplayer], [Lightyear],
  )
