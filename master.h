#pragma once
#include "phys/world.h"
#include <filesystem>
#include <ctime>

// MASTER SYSTEM
// controls the flow of the entire simulation, and is responsible for the main loop of the program.

class Master_sys
{

	tm sim_time; //time in the simulation starting 
	World_subsys World;	
	int simticks = 0; // ticks since simulation start.
	float time_mult = 1.0f; // used to slow down or speed up simulation.
	float time_step = 0.02f; //time that should pass per tick. The tick rate should preferrable be synced to this time.
	bool worldloaded = false; // Did the World load successfully?

public:

	Master_sys();
	void Init(std::filesystem::path path, std::string file);
private:
	bool stop_signal = false;
	int Loop();
	int Load_World_Data(std::filesystem::path path, std::string file);



};