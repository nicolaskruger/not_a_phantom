#  Mass Flux

# DESIGN DOCUMENT

By Aizin for the 2022 Pirate Software Game Jam  
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

## **![][image1]** {#heading}

## **Introduction**

### **Game Summary Pitch** {#game-summary-pitch}

	Mass Flux is a tile based puzzle game about reducing or generally manipulating the shape of a mass of cell-blocks to navigate to a goal.

### **Inspiration** {#inspiration}

**Snakebird**  
Snakebird provides the ingenuity for the player to control **a body that isn't always helpful** and will certainly get in the way more often than aid one’s plan. A long body in that game may be susceptible to support the player from falling but often can **restrict and block certain movement** making puzzle areas much harder  to navigate. 

**Baba Is You**  
Baba is You is the main inspiration towards the style and mood of the game. Especially graphics-wise to fit the 2-bit graphical prompt, the **minimalist art style** will hold useful to act as a base. Otherwise, the gameplay also further provides inspiration towards level design as an abstract puzzle game.

### **Player Experience** {#player-experience}

In a single screen dungeon for each of the **15 levels**, the player will solve a short but perhaps complex puzzle **requiring planning and management**. The player must learn and use their knowledge of the interactions between the player cells and various environmental items to understand how to pass through each level.

### **Platform** {#platform}

The game is developed to be released on windows PC  
	

### **Development Software** {#development-software}

- Gamemaker Studio version 2.3 for programming  
- Aseprite for graphics and UI  
- FL Studio 12 for all music and SFX

### **Genre** {#genre}

Singleplayer, puzzle, casual

### **Target Audience** {#target-audience}

Without heavy or complicated ideas, and intuitive-to-grasp mechanics, this game is marketed to at least **casual game players** who are up for puzzling challenges as well as more veteran players up for solving complicated problems

## **Concept** {#concept}

### **Gameplay overview** {#gameplay-overview}

The player controls a mass of player cells, each with **individual status**, but **moves as a collective**. Individual cells may die or be created which influences the total shape of the mass. By navigating through each level, the player must **strategically manipulate the shape** of the mass to be able to pass around or through obstacles to reach the goal. 

### **Theme Interpretation (Sacrifice Is Strength)** {#theme-interpretation-(sacrifice-is-strength)}

**‘Sacrifice’ interpretation \- The player voluntarily offers something they would otherwise use to their benefit to then gain something else of use in its stead.**

Within the context of a puzzle game, rather than a sacrifice strictly being an optional upgrade of sorts, the timing, placement, and orientation a ‘sacrifice’ within this game instead occurs to allow the solution or progression of the puzzle. Only through careful planning of movement to remove parts of the player’s mass can the player make their way to the exit. ***One must often sacrifice a part of the player mass to pass through specific areas as they may be too large or encompass the wrong shape.***

### **Primary Mechanics** {#primary-mechanics}

| Mechanic | Animated Mockup  *(Art not necessarily final)* |
| ----- | ----- |
|         **Walls**         a A stopping force to prevent a player too large to access a certain area. Otherwise to simply restrict movement. | ![][image2] |
|         **Spikes**        a When a player cell walks on top of a spike, that cell will die and further simplify the player mass. | ![][image3] |
|         **Holes**       a The player mass can walk freely over a hole as long as at least one cell is on a floor tile. If the entire mass is over the hole, the entire player mass dies. | ![][image4] |
|         **Fruit**       a If a player cell moves over a fruit, it will eat the fruit and generate a new cell on the ***opposite side*** of the mass it is a part of. | ![][image5] |

	

### **Secondary Mechanics** {#secondary-mechanics}

| Mechanic | Animated Mockup  *(Art not necessarily final)* |
| ----- | :---: |
|         **Independence**        a If two player masses happen to separate, they will still move synchronously but interact with the environment independently. If then connected again, the two masses will join to act as one. | ![][image6] |
|         **Set Spikes**        a When a player cell walks on top of a set spike, ***after moving off*** of it, it will then become a regular spike trap | ![][image7] |

## **Art** {#art}

### **Theme Interpretation** {#theme-interpretation}

While maintaining the very limited color palette theme, the sole use of black in white seems way too common, and a bit harsh as a color scheme for a relaxing puzzle game. To circumvent this, a soft, **dark blue color** will act as the unique accent color as opposed to black with white being the primary, carrying color to base the sprites off of.

### **Design** {#design}

A very **minimalistic** approach will go into the design of the game, heavily relying on the severe contrast of the limited colors to provide detail. Though, the design still is clean and smooth in the sense that, the use of many shades of a color will not be as present to confront the **retro style** and pixel art.

![][image8]  
*\*Not an actual puzzle/level, merely a full example of the tileset and art style\**

## **Audio** {#audio}

### **Music** {#music}

To add to the overall theme and vibe of the game, there will be minimalism incorporated into the music. Heavy use of reverb and effects to fill space within the few instruments. Bass and drums will generally constitute the majority of tracks with accompanying softer sounds. Mainly through synthesized sounds rather than acoustic will further suggest the retro style.

### **Sound Effects** {#sound-effects}

To add more flare and polish to the experience, a multitude of environmental sound effects will give weight and feedback to the player’s actions. Rather than foley, or otherwise realistic sounds, synthesized blips, bloops, and whooshes are used.

## **Game Experience** {#game-experience}

### **UI** {#ui}

On top of the rigid pixel art constituting the rest of the art, a more smooth, higher definition style will be incorporated in the UI. Utilizing many shades of white and black allowed in the art restriction, anti-aliasing is used to further emphasize the UI.

### **Controls** {#controls}

**Keyboard**  
	Arrow keys / WASD  
**Gamepad**  
	Dpad

## **Development Timeline** {#development-timeline}

**MINIMUM VIABLE PRODUCT**

| \# | Assignment | Type | Status | Finish By | Notes |
| :---: | :---- | :---- | :---- | :---- | :---- |
| 1 | Design Document | Other | Finished | Jun 22, 2022 |  |
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

[image1]: <data:image/png;base64,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>

[image2]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAKAAAACgAgMAAAD2oY++AAAADFBMVEUAAAAAHjj7/f8AAADWoIR9AAAAAXRSTlMAQObYZgAAAalJREFUeF7lmEuuwyAMRdOnenWZMOkqM8mkq2PycMjHdmKLOggl7ZWqlvboCrAxTh+v7lgQxn4k4z/y2VRjMJDPJkhlgJEu2gK5nvKLTdDTUbFjQzAOfKyCUioIIq1UUMoEfzEfua4J3uPMuLJnnSPkwLKlBjL43LHPpw7WX4SKHQ3wHpGh8oDB2EWf417nsudQrxSeVCeH9H7eMQ5pCzCj5pfHMa0xbqPUAaQpdmOIuLkRih2NyBD7zjfHN+7aNpwDs1xgHkd5+01XF655xCNf7NgQ5LtogFKP5Vo+qmZUxY71wab1ketbQXkKuTyOtr4V5PVRyuNYKTJWfWTyOMr6yFXseAdwXfU7F0p1J4sd64N6fZznDNN9/UFPoYIyNVVQqj6o9j3b3Z/lcUyNDrCDHfD27wG7AF/fI8V30gC51FVjt4frXvLzvCN2e1NHiq1B6vk8jgf1ESc5FZChfI4E3NfH3LhkFTvevD5e5imufn1cO9K9oPL/j/iflCt7DPAykWkM1o0MqnZkuGqAvxprqvOgrJYqKKWCeK5r56Mz1gY4P3HPMkCufy7IZHDCVCblAAAAAElFTkSuQmCC>

[image3]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAKAAAACgAgMAAAD2oY++AAAADFBMVEUAAAAAHjj7/f8AAADWoIR9AAAAAXRSTlMAQObYZgAAAU9JREFUeF7tWMsOgyAQxEa+zgsXvtILF7+OS5eHisQ1Ky4pWkeDrJ1MhIEt0GlBwyd/geElHuJRxF4qK4WwRsAwGrWwExKTFTv28dhro4S/3acIKJCYrMhPrNBqqdzDuHItdmKy4h2IeD8qE64YkhX7uRL60w3tfZxXHEb/cIMwVCLmmKx4B+LiTLRA2vW3DciKCVElvejDcEWUKB7jOrFNr8EazBZxQjHxGtyWltXrSethjYq9XmYhNDnMQwRkRZTYkNdLq6dg0fVstnhdPz/mICv+PTHLjxlKFI/xVCJTfuR3JslmMLPROVimWDs/5vghEc+PpetHfiKaHxv6n+EnvuvHDdr0mik/vuvHDRryGs+PGciK+I49A1mxd4rz0c6mmsVkxTsQa/RjPCeCvgo3EpMV+YkVWg37mNEdXvoNDTR2QmKyIv83vsRDPIr4BU9jgvtWrKixAAAAAElFTkSuQmCC>

[image4]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAMAAAACgAgMAAAAnRovOAAAADFBMVEUAAAAAHjj7/f8AAADWoIR9AAAAAXRSTlMAQObYZgAAAT1JREFUeF7tmE0OhCAMhdXI6dy48ZSzcePp2IwKrYw20PKTSWb6uSjRPB4FtMF+6WQM9xspVMBBBRxUwEEFHFTAYYSGmc9gV7pgvKCR7zC5PozBRx/Mq2+IHb4iWMkLIARxeph2Pw2mc+txx9aYpfnoP5WC3CHIYc/C2D0Ha4gLTcQOuJeQYLwBEzQqOEz34fskPCUO++CPfrbrTkCVHM51CPqiKXHYlnNP0uuAe7jEwbrvBv0+ICUOnto5tBdgDpt79Sy9lS7kDlAXUj0DcgeoCxDDhxRiBxVwGLAuYIyT4yDjXwW+LmCMk+PQeKVHrAsY4+Q4+LqAMY7Y4RcEI9aFZvUBZ9/H1HYSO6iAgwo4qICDCjg8BeQJ5foePgUJHicUa8ifM/n/ZvDEzkXsoAIOKuCgAg4q4KACDu0FbzIpXo74WPo+AAAAAElFTkSuQmCC>

[image5]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAL4AAAC+AgMAAAD2Gl0OAAAADFBMVEUAAAAAHjj7/f/////WHqJvAAAAAXRSTlMAQObYZgAAAlhJREFUeF7tmcFugzAMhl3WE09SsedZVW0XtGMfokJ7iJ5zRLzPEO+w0y5cJqElMZQkgOyUptqq/EKAKV8NdnFIvXkBL/0k7hFKEeAoAhxtoQLYV9AeRZsC1EVSQvdaQl0VzW6w34U6JRfyTH8Pm+DPw7ZSlw5yJS+xKLoPfenQ7OQeLtrO1R2qxd9DeOAeUVIhgH4lg2KujF0dS7n+9vbwCED4PDCArDQtBmCLA1jncABLT8/ukYm+Pr9Go/P2QANJ2YAwzHGXJxJIyuSUpmK0x494IoFdA5kA/bxpkYArEuisXxIDcEUCSeHYtkkrENCOuzzAEA8Y88YEDJFAo1b5eBMk4IoFGEHiAaZIoCugy0V+sUnAFQ003SFdk+nw5X7rHsgKvalxM9VqD8l+r7fqnWdW3h4iwNEUULVxMQtzAKEIcDQFZCHCgjqvKUBoCtw60+4zLe+hqpZrxk08QI11Y0HeHh4BcKOksqC0mInrPfRvmjU6MJWdDoZ1vYcz5jebGR+sL/X2wADsgDEAWzTwr6YDAqPvVI04HYA7AzmOC3Pjw5q3SjLTWmvmD0ZdOuK44IwPOH84X+w1Htp+XJiOD3H+MMwfLtW7wC1uJvKfP4QHOOODJW8PEeDIAMRy1TC0xgNPDwbky+Xe0BoPIkziyHLvao2H5XJvytvDIwDhy/0d/pDCbhn2efpGmbLHXbSHXlHsA3F0RR4EdiN111K13mTY9TK04tDWXcu/2o+7Q5T0oyaDcNQ96O6g/j+VwZFPnmpToy0jhC1saN+8PYS/hwhwFAGOfgFX5t9UWzQAbgAAAABJRU5ErkJggg==>

[image6]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAMAAAACgAgMAAAAnRovOAAAADFBMVEUAAAAAHjj7/f8AAADWoIR9AAAAAXRSTlMAQObYZgAAAb1JREFUeF7tl01ywyAMRkmnOp033nBKb9j4dGwKVnBiIUDYVtzM+G0yMPP6lV+ZhzVbYHTDPIB3pD/xQztasELxz5uCUOOXdkRgoD0vzkjwDsxEO1e6ExgBRmPsSHsTjFDnAsGXJ2ghE1pkAtiv20u3IOESwbtT9xIjwBhPdQlGqMPuVqN8L0W++16KnLrS7Dro3hqfuVtrZEKLS4TaOrNCHWalYZi1TxxonzhvTGWeGKEOM0uVKTLnJFhXnCKzI+ECweNPcSUyocWjcYQzuhOydXjWt/XYre3nv3I8YcCdBLSdOroT9IVsDGVwMJoJZgwz5boTGGF8W4X3NnYyQh3xGPyEby+9hPAKDheKZsLCtCOBEdx6ddB2PHyMUIcRSiuNMEIdZpZmuy3V2/YZCfSLKbX9UjfOSCiBX4LdCfpCNoYZb+l15lN7NliWjifQVXgnHrrjCRRaL7oT/pmwax26hF3nQSDgrZF2gEDYIhbSy0UsJPSF5m594R2EjyeNhFQfYHChBGkkhJMMGGH3fGsIElJ9wEKnkbDeVMuvRkICXy7dCfpCcwy0XnQn6L9QbkHCLUi4BQm3IOEWJPwBV4xeRuroRs8AAAAASUVORK5CYII=>

[image7]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAKAAAABgAgMAAADnSZEKAAAADFBMVEUAAAAAHjj7/f8AAADWoIR9AAAAAXRSTlMAQObYZgAAALdJREFUeF7tlDEShCAMRXHHnM6GhlPabMPpaJY1ijtxks1gGjWPAsHPn5FvMqSAQFym8l7XlBfd4Bi3h2leJmhvCGrHxwujcIt9jjJ3FdZ/u+wrSo+jUTKtZkLIia/BPseCdcigdryCsH11xrbG3qTa0V54yf4ICQfdPwg5hu1oxEiAK0S144/QqAofL8T+aJl1TRvK+ay9P4qohYf+mL8R1UGvQO3Ysv6H2tGFIi4UcaGIC0XshR/xeiVcFXWXAAAAAABJRU5ErkJggg==>

[image8]: <data:image/png;base64,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>