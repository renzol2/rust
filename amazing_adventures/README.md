# Amazing Adventures (Rust)

A Rust implementation of a UIUC CS 126 assignment, [**Amazing Adventures**](https://courses.grainger.illinois.edu/cs126/sp2021/assignments/amazing-adventures/).

## Steps

- [x] Load JSON file into data structure
- [x] Allow user to pick room from `resources` folder
- [x] Implement room traversal
- [x] Implement winning
- [x] Implement quit command
- [x] Add tests
- [x] Refactor `AdventureRoom` with item interface (finding, taking, and dropping items)
- [x] Implement picking up and dropping item commands
- [x] Add commands to view bag and inspect inventory
- [ ] (Reach) Implement some battle mechanic, for fun

## Reach mechanic - Pokemon Battles

To better practice Rust, I will be implementing Pokemon battles in Amazing Adventures. Here is a high level idea of the new features:

- The player will choose a starter Pokemon before the adventure begins, and the starter Pokemon will be in the player's party.
- When traversing through rooms, there will be a random chance to encounter a wild Pokemon. The Pokemon battle mode will be the same as the mainline Pokemon games, where the player will fight the wild Pokemon with the Pokemon in their own party. The player can also use items to heal their Pokemon or catch the wild Pokemon.
- Players can catch wild Pokemon and add them to their party.
- When players reach the final room, they will encounter a trainer that they have to fight in order to beat the Adventure. If the player wins the battle, they win the adventure. If they lose, they will have to try again.

### TODOs

- [ ] Implement starter Pokemon & `party` command to view party
- [ ] Implement random encounters of Pokemon
- [ ] Implement battling wild Pokemon
- [ ] Implement catching wild Pokemon
