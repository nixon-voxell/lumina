#set text(size: 12pt)
#set heading(numbering: "1.")
#show link: set text(style: "italic", fill: gray.mix((black, 50%)))

#show outline.entry.where(level: 1): it => {
  v(12pt, weak: true)
  strong(it)
}

#align(center)[
  #text(size: 24pt)[*Lumina*]\
  _Game Design Document (GDD)_
]

= Quick Summary

Lumina is a top down fast paced objective based PvPvE game.
Players in a team of 3 will be tasked to control a spaceship to fight mobs, collect motes, and gain domninance over the opposing team.

- *Platform*: PC Native (Windows/Mac/Linux)
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
- *Genre*: 2D, Co-op, PvP, PvE, Top-down, Objective-based PvPvE

= Setting

Set on Luminara, a planet that is currently in the middle of an energy crisis war.
The inhabitants of the planet (_Luminites_ and _Luminids_) ventures into the void to fight for resources (_Lumina_).

== Tone

The world of _Luminara_ strikes a balance between whimsical and serious, blending playful mischief with an undertone of cunning ambition.
While the characters, _Luminites_ and _Luminids_, have a lighthearted, almost impish charm, their actions are driven by a calculated desire to outwit and outperform each other.
This creates an atmosphere reminiscent of _"Despicable Me"_, filled with sly humor and mischievous antics, but layered with stakes that emphasize clever strategies and resourceful thinking.

== Character Traits and Motivations

Even though the _Luminites_ and _Luminids_ are in a war, they were actually once the same species, living in harmony.
Not all _Luminites_ and _Luminids_ harbor genuine hatred for one another, many are driven to battle by the ambitions and orders of their leaders and politicians..
This struggle mirrors our own world, where chaos and conflict are often incited by those in power.
The never-ending war rages on, even as their home planet, _Luminara_, cries out in agony...

#pagebreak()

#outline()

#pagebreak()

= Game Loop

#figure(image("game loop.png"), caption: "Game Loop")

*From a player's perspective:*

As a player, your goal is to gain dominance of the *_Tesseract_*.
This is achieved by moving the effect bar towards the opposite side.

== Core Loop

+ Start the game with a team of 3.
+ Kill *_Dauntless_* (mobs) to obtain *_Lumina_* (currency).
+ Feed the *_Lumina_* into the *_Tesseract_* (depositor) to increase your team's dominance.
+ Team with total dominance win, get the *_Lumina_* resource, and head back to *_Luminara_* (home planet).

== Death Loop

+ Get killed by mobs / opponents.
+ Get a death waiting penalty (increased gradually as game duration increases).
+ Respawn at a random location with a 5 secs immunity.

#pagebreak()

= Game Mechanics

#pagebreak()

= Environment Design

== Character Design

Characters are aimed to represent the form of a light bulb / luminance source.

#pagebreak()

= Inspirations

== Destiny 2 (Gambit game mode)

#figure(
  image("inspirations/destiny 2 gambit.jpg"),
  caption: "Destiny 2 gambit mode",
)
#link("https://www.youtube.com/watch?v=TSBdw668c4I")

== Neon Chrome

#figure(image("inspirations/neon chrome.png"), caption: "Neon Chrome")
#link("https://www.youtube.com/watch?v=puhw1bEVtro")

== Intravenous 2

#figure(image("inspirations/intravenous 2.jpg"), caption: "Intravenous 2")
#link("https://store.steampowered.com/app/2608270/Intravenous_2/")
