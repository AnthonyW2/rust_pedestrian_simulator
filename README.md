# Micro-scale Pedestrain Simulation
Written in Rust, using Raylib to display the simulation.

## Background
This is a very basic simulation I made in about a week for a university unit.  
Being my first project using Rust, and my first time using Raylib, the code is a bit of a mess.

## Explanation
The goal of this simulation is to reasonably accurately simulate pedestrains walking in two directions through a constricted space.  
The simulation was calibrated using data collected in a specific location (with a 25 metre walkway, 6 metres wide), so it probably doesn't represent any other situation well.  
The simulation is stochastic - pedestrians are created in random positions with random targets, and random noise is added to their movement.

The specific reason why the simulation was made was to test the impacts of different pedestrian "etiquettes" - in this case, how average travel time differs when everyone stays left or when everyone walks randomly.

