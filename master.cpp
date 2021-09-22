#include "phys/world.h"
#include "master.h"
#include <nlohmann/json.hpp>
#include <filesystem>
#include <iostream>
#include <fstream>
#include <Windows.h>


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
		
		std::filesystem::current_path(path);
		nlohmann::json jparser;
		std::ifstream input_stream;
		input_stream.open(file, std::ifstream::in);
		std::stringstream output;
		if (input_stream.is_open()) {
			std::cout << "file opened! \n";
		}
		else
		{
			std::cerr << "File Failed to open! \n";
			throw std::invalid_argument(file);
		}
		try
		{
			jparser = jparser.parse(input_stream);
			std::cout << "Success!";
		}
		catch (nlohmann::json::parse_error& ex)
		{
			std::cerr << "parse error at byte " << ex.byte << std::endl;
		}
		std::string name = jparser["name"];
		World.Create_World(name, jparser["Bodies"]);
		input_stream.close();
		return 1;// oh god oh fuck
	}


