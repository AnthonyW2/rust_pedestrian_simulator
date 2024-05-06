# Micro-scale Pedestrian Simulator
Written in Rust, using Raylib to display the simulation.

## Background
This is a very basic simulation I made in about a week for a university unit.  
Being my first large Rust project, and my first time using Raylib, the code is a bit of a mess.

## Explanation
The goal of this simulation is to reasonably accurately simulate pedestrains walking in two directions through a constricted space.  
The simulation was calibrated using data collected in a specific location (with a 25 metre walkway, 6 metres wide), so it probably doesn't represent any other situation well.  
The simulation is stochastic - pedestrians are created in random positions with random targets, and random noise is added to their movement.

The specific reason why the simulation was made was to test the impacts of different pedestrian "etiquettes" - in this case, how average travel time differs when everyone stays left or when everyone walks randomly.

## Examples
There are 4 environments & simulations included that are based on the location where data was collected - these work the best.  
There are also 3 demonstration simulations: a basic debugging environment; a diagonal pathway; and a crossroads. This simulation was not built with crossroads in mind, so there are problems with the last one, but it does work.

### Calibration simulation:
![image of calibration simulation](https://raw.githubusercontent.com/AnthonyW2/rust_pedestrian_simulator/master/images/calibration_1.png?raw=true)

### Diagonal path:
![image of diagonal path simulation](https://raw.githubusercontent.com/AnthonyW2/rust_pedestrian_simulator/master/images/diagonal_1.png?raw=true)

### Crossroads (experimental):
![image of crossroads simulation](https://raw.githubusercontent.com/AnthonyW2/rust_pedestrian_simulator/master/images/crossroads_1.png?raw=true)
