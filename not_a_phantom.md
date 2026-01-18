#  Not a Phantom

# DESIGN DOCUMENT

By Ni for the 2026 Pirate Software Game Jam  
---

[**Introduction**](#heading)	**[2](#heading)**

[Game Summary](#game-summary-pitch)	[2](#game-summary-pitch)

[Inspiration](#inspiration)	[2](#inspiration)

[Player Experience](#player-experience)	[2](#player-experience)

[Platform](#platform)	[3](#platform)

[Development Software](#development-software)	[3](#development-software)

[Genre](#genre)	[3](#genre)

[Target Audience](#target-audience)	[3](#target-audience)

[**Concept**](#concept)	**[3](#concept)**

[Gameplay overview](#gameplay-overview)	[3](#gameplay-overview)

[Theme Interpretation (Sacrifice Is Strength)](#theme-interpretation-\(sacrifice-is-strength\))	[4](#theme-interpretation-\(sacrifice-is-strength\))

[Primary Mechanics](#primary-mechanics)	[4](#primary-mechanics)

[Secondary Mechanics](#secondary-mechanics)	[5](#secondary-mechanics)

[**Art**](#art)	**[6](#art)**

[Theme Interpretation](#theme-interpretation)	[6](#theme-interpretation)

[Design](#design)	[6](#design)

[**Audio**](#audio)	**[7](#audio)**

[Music](#music)	[7](#music)

[Sound Effects](#sound-effects)	[7](#sound-effects)

[**Game Experience**](#game-experience)	**[7](#game-experience)**

[UI](#ui)	[7](#ui)

[Controls](#controls)	[7](#controls)

[**Development Timeline**](#development-timeline)	**[8](#development-timeline)**

## **Introduction**

### **Game Summary Pitch** {#game-summary-pitch}

	Run and do not let the norms see you  

### **Inspiration** {#inspiration}

**Seleste**  
Help Madeline survive her inner demons on her journey to the top of Celeste Mountain, in this super-tight, hand-crafted platformer from the creators of multiplayer classic TowerFall.

### **Player Experience** {#player-experience}

You are a ghost of a based edge boy, and you don't want to no norms to see you so be fast an d invisible.

### **Platform** {#platform}

The game is developed to be released ONLY ON Lixus 🐧😎 and chrome browser 🤮   
	

### **Development Software** {#development-software}

- Rust + Bevy

### **Genre** {#genre}

Singleplayer, puzzle, platform

### **Target Audience** {#target-audience}

Norms not allowed

## **Concept** {#concept}

### **Gameplay overview** {#gameplay-overview}

It is a 2d platform and in each stage you have to throw pass norms and not been seen.

### **Theme Interpretation (do not be seen)** {#theme-interpretation-(sacrifice-is-strength)}

When you are and edge boy all things you do must be unique, so for the norms not copy you, You must not be seen.

### **Primary Mechanics** {#primary-mechanics}

| Mechanic | Animated Mockup  *(Art not necessarily final)* |
| ----- | ----- |
|         **Walls**         you can climbe. |  |
|         **Norms**        do not be seen. |  |
|         **Dash**       go fast in a little time. |  |
|         **Invisible**       not seen and not touched. |  |

	

### **Secondary Mechanics** {#secondary-mechanics}

Don't have

## **Art** {#art}

### **Theme Interpretation** {#theme-interpretation}

Not done yet

### **Design** {#design}

Not done yet

## **Audio** {#audio}

### **Music** {#music}

No music

### **Sound Effects** {#sound-effects}

No sound

## **Game Experience** {#game-experience}

### **UI** {#ui}

Just play start if you are seen you lose

### **Controls** {#controls}

**Keyboard**  
  hjkl  VIM like move 
  w   Dash
  e   Ghost Mode   
**Gamepad**  
	none

## **Development Timeline** {#development-timeline}

**MINIMUM VIABLE PRODUCT**

| \# | Assignment | Type | Status | Finish By | Notes |
| :---: | :---- | :---- | :---- | :---- | :---- |
| 1 | Design Document | Other | Finished | Sun Jan 18 07:39:35 PM -03 2026 |  |
| 2 | Create player and wall assets | Art | Finished | Jun 23, 2022 | Prototype for GDD is done |
| 3 | Main menu theme | Audio | Finished | Jun 23, 2022 | Can be really short, player won’t be on main menu for long |
| 4 | UI / Main menu | Coding | Finished | Jun 23, 2022 | Button UI, screen transition, title screen |
| 5 | Level theme | Audio | Finished | Jun 24, 2022 | Should be more substantial and not annoying |
| 6 | Simple player movement | Coding | Finished | Jun 23, 2022 | Move single cells around and collide with walls |
| 7 | Complex player movement | Coding | Finished | Jun 24, 2022 | Multi cell masses act together to collide with walls |
| 8 | Spikes and holes with player interactions | Coding | Finished | Jun 24, 2022 | Implement spike and holes mechanics |
| 9 | Fruit interaction | Coding | Finished | Jun 25, 2022 |  |
| 10 | Special effects | Art | Finished | Jun 25, 2022 | Dust particles during movement |
| 11 | Player animation | Art | Finished | Jun 25, 2022 | Idle blinks, movement polish |
| 12 | Sound effects | Audio | Finished | Jun 25, 2022 | Player movement, UI interaction |
| 13 | Pause menu | Coding | Finished | Jun 26, 2022 | Access to the main menu or resetting the level |
| 14 | Level select menu | Coding | Finished | Jun 26, 2022 |  |
| 15 | Level design (1-7) | Other | Finished | Jun 27, 2022 | Create levels 1 through 7 |
| 16 | Level design (8-15) | Other | Finished | Jun 28, 2022 | Create levels 7 through 15 |
| 17 | Any extra polish | Other | Finished | Jun 29, 2022 |  |
| 18 | SUBMIT | Other | Finished | Jun 29, 2022 | Create Itch Page and upload |

**BEYOND (if ahead of schedule / extra time)**

| Undo | Other | Not started | At any point, the player may undo their move, any movement, creation, or destruction of a player cell is reversed |
| :---- | :---- | :---- | :---- |
| Extra levels | Other | Not started |  |
| Settings Menu | Coding | Finished | Volume slider, fullscreen toggle |
