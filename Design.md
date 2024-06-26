# Design of the game

## Goal

This is going to be a very simple Game for adukts and children: Memory trains your short term memory: The game
starts with randomly placing pairs of cards with pictures (in this first version just numbers) in a rectangular
matrix (currently 8x8). The picture is downwards and you only see the backside.
The players take turns. So the game must allow to enter the number of players. The players choose two cards when
it is their turn. These two cards are then shown openly. If they show the same picture the player found a pair
and keeps them. Otherwise the players should memorize which card was where. The cards get turned back around so
that you only see the backside and it is the next players turn.
When there are no cards anymore on the table, the game is over. The player with the most cards wins.

## Long term extension ideas

- Make it possible to create your own card sets
- Play over the network with other players (there could be multiple players at each participating computer)
- Have an AI player with different levels of intelligence (or memory capacity).

## Highlevel Design

- We use the model-view-controller pattern
- The model is realized in the struct Game (found in lib.rs)
- The view is realized in the struct Renderer in the file board_view.rs
- The controller is realized in the struct Control in file controller.rs

Game hosts all game logic and data (players, card deck, board).

Renderer hosts all functions to display the game state on the screen.

Control hosts all functions to respond to input from the user (mouse click on a card should reveal the card). It
connects the input to the Game state and calls the game logic functions accordingly.
This struct (was about to call it object or class) collects the events from the event loop and handles them by
calling the corresponding game logic.

<div hidden>

```plantuml

@startuml mvc-pattern

skinparam interface {
  backgroundColor RosyBrown
  borderColor orange
}

skinparam component {
  FontSize 13
  BackgroundColor<<model>> Pink
  BorderColor<<model>> #FF6655

  BackgroundColor<<view>> LightBlue
  BorderColor<<view>> DarkBlue

  FontName Courier
  BorderColor black
  BackgroundColor gold
  ArrowFontName Impact
  ArrowColor #FF6655
  ArrowFontColor #777777
}

:User:

folder lib_file {
    [Model] as model << model >>
    () "Game" as game
}

folder view_fldr {
    [View] as view << view >>
    () "Renderer" as renderer
}
folder ctrl_fldr {
    [Controller] as controller
    [Evt Loop] as loop
    () "Control" as control
}

controller - loop

game -d- model
view -r- renderer
control -d- controller

User --> control
controller ..> game : write
renderer <. controller : "trigger"
view ..> game : read
@enduml

```

</div>

![Model View Controller Pattern](mvc-pattern.svg)

Both View and Model are easy to understand. The Model consists of some
structs that represent the domain of the game (cards, players, field, etc.).
The View simply renders the objects of the Model to the screen. This is
basically it. The rest is just SDL2 stuff.

The Controller is slightly more complex because it realizes a state machine
which is outlined below:

```mermaid
stateDiagram
    state "start game" as start_game
    state "user chooses first card" as first_card
    state "user chooses second card" as second_card
    state "user views result" as view_result
    state "next user" as next_user
    state "game over" as game_over

    [*] --> start_game
    start_game --> first_card: any click
    first_card --> second_card: card click
    second_card --> view_result: card click
    view_result --> next_user: any click game not yet over
    view_result --> game_over: any click and game over
    game_over --> [*]: Close Window
    game_over --> start_game: any click
    next_user --> first_card: any click
```
