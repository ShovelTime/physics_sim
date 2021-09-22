#pragma once
#include "phys/world.h"
#include <filesystem>

// MASTER SUBSYSTEM
// controls the flow of the entire simulation, and is the main loop of the program.

class Master_subsys
{
	World_subsys World;	
	float timemult = 1.0f; // used to slow down or speed up simulation.
	float timestep = 0.02f; //time that should pass per tick. The tick rate should preferrable be synced to this time.
	float tickrate = timestep; // starts at timestep, can be slowed down if calculation takes too long.
	bool worldloaded = false; // Did the World load successfully?

public:
	Master_subsys();
	void Init(std::filesystem::path path, std::string file);
	int Load_World_Data(std::filesystem::path path, std::string file);


};