#include "phys/world.h"
#include "master.h"
#include <nlohmann/json.hpp>
#include <string.h>
#include <filesystem>
#include <iostream>
#include <fstream>
#include <Windows.h>

using nlohmann::json;
// MASTER SUBSYSTEM
// controls the flow of the entire simulation, and is the main loop of the program.

/*
	World_subsys* World;
	float timemult = 1.0f; // used to slow down or speed up simulation, 1.0 is real-time.
	float timestep = 0.02f; //real-time that should pass per tick. The tick rate should preferrable be synced to this time.
	float tickrate = timestep; // starts at timestep, can be slowed down if calculation takes too long.
	*/
	 
	Master_subsys::Master_subsys() 
	{

	}
	void Master_subsys::Init(std::filesystem::path path, std::string file)
	{
		Load_World_Data(path, file);
		
	}
	int Master_subsys::Load_World_Data(std::filesystem::path path, std::string file)
	{
		std::ifstream input_stream(path / file);
		if(input_stream.is_open())
		{
			try {
				json jparser = nlohmann::json{}.parse(input_stream);
				std::string name = jparser.at("World").at(0).at("name").get<std::string>();
				

			}
			catch (nlohmann::json::exception ex)
			{
				std::cout << "Json Error!" << std::endl << ex.what() << std::endl;
			}
		}


		return 1; 

	}


