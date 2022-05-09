# Physics Simulation in Rust

This program is an n-body simulation of the main bodies of our solar system, using Cowell's formula to calculate the bodies's orbital state vectors.

## Compiling
Compiling is done using the `cargo build` command

`cargo build`

For performance related reasons, it is recommend to build the program in release mode

`cargo build -r`

## Running

To run the program, ensure that the `Sol.Json` file is located in the same folder as the program's executable.

## Modifying Sol.Json

The bodies included in the default JSON can be replaced by any other orbital data, the JSON follows the following Format:

```
{
  "Info" //General info about the system itself
    {
      "name" : //Name of System
      "date" : //Currently unused, used to track the beginning date of the simulation
    }
    "Bodies" //Contains data about the orbital bodies to be simulated
    [
      { //Example of an Orbital Body
        "name": "Sol", // Name of orbital object
        "radius": 696342000, //radius of object (meters - m)
        "mass": 1988500e+24, // mass (kg)
        "location": [ -1.068108951496322e+09, -4.177210908491462E+08, 3.086887010002915E+07 ], // Position in the x,y,z axis respectively (meters)
        "velocity": [ 9.305302656256911E+00, -1.283177282717393E+01, -1.631700118015769E-01 ] // Ditto, but for velocity (meters/second)
      }
    ]


}

```
