#include "phys/world.h"
#include "phys/bodies.h"
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
				json bodyparser = jparser.at("World").at(0).at("Bodies");
				std::vector<Body> Bodies;
				int size = bodyparser.size();
				for (int i = 0; i < size; ++i) //Parse Body data into Objects
				{
					std::cout << i << std::endl;
					Bodies.push_back(
						Body(
						bodyparser.at(i).at("name").get<std::string>(),
						bodyparser.at(i).at("radius").get<float>(),
						bodyparser.at(i).at("mass").get<double>(),
						bodyparser.at(i).at("location").get<std::vector<double>>(),
						bodyparser.at(i).at("velocity").get<std::vector<double>>()
						)
					);
				}
				std::cout << "yup" << std::endl;
			
			}
			catch (std::exception ex)
			{
				std::cout << "Error Parsing Json File!" << std::endl << ex.what() << std::endl;
				return -1;
			}
		}


		return 1; 

	}


