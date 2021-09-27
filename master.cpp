#include "phys/world.h"
#include "phys/bodies.h"
#include "phys/phys.h"
#include "master.h"
#include <nlohmann/json.hpp>
#include <string.h>
#include <filesystem>
#include <iostream>
#include <fstream>
#include <future>
#include <Windows.h>

using nlohmann::json;
// MASTER SYSTEM
// controls the flow of the entire simulation, and is the main loop of the program.

/*
	World_subsysWorld;
	float timemult = 1.0f; // used to slow down or speed up simulation, 1.0 is real-time.
	float timestep = 0.02f; //real-time that should pass per tick. The tick rate should preferrable be synced to this time.
	float tickrate = timestep; // starts at timestep, can be slowed down if calculation takes too long.
	*/
	 
	Master_sys::Master_sys() 
	{
	}
	void Parse_Date(std::string date) 
	{

	}
	void Master_sys::Init(std::filesystem::path path, std::string file)
	{
		if (!Load_World_Data(path, file)){

		}
		worldloaded = true;
		std::cout << "Init Complete" << std::endl;
		std::cout << phys::get_distance_num(World.GetEntities()[0].position, World.GetEntities()[2].position) << " km";
		return;
		Loop();


		
	}
	int Master_sys::Load_World_Data(std::filesystem::path path, std::string file)
	{
		std::ifstream input_stream(path / file);
		if(input_stream.is_open())
		{
			try {
				json jparser = nlohmann::json{}.parse(input_stream);
				std::string name = jparser.at("World").at(0).at("name").get<std::string>();
				json bodyparser = jparser.at("World").at(0).at("Bodies");
				std::string date = jparser.at("World").at(0).at("date").get<std::string>();
				Parse_Date(date);
				std::vector<Body> Bodies;

				
				for (int i = 0; i < bodyparser.size(); ++i) //Parse Body data into Objects
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
					Body res = Bodies.back();
					if (res.mass < 0.0 || res.radius < 0.0) 
					{
						throw std::invalid_argument("Body nr " + std::to_string((Bodies.size() - 1)) + "; Mass or Radius should not be Negative!\n Name: "+ Bodies.back().name +"\n Mass: " + std::to_string(Bodies.back().mass) + "\n Radius: " + std::to_string(Bodies.back().radius));
					}
					if (res.velocity.size() != 3 || res.position.size() != 3)
					{
						throw std::invalid_argument("Body nr " + std::to_string((Bodies.size() - 1)) + "; Position or velocity vector has wrong size! " + Bodies.back().name + "\n position size: " + std::to_string(Bodies.back().position.size()) + "\n velocity size: " + std::to_string(Bodies.back().velocity.size()));
					}
				}
				if (World.Create_World(name, Bodies)) {
					return 1;
				}
				else
				{
					throw E_NOTIMPL; // replace this ex with something else
				}


				std::cout << "yup" << std::endl;
			
			}
			catch (nlohmann::json::exception ex)
			{
				std::cout << "Error Parsing Json!" << std::endl << ex.what() << std::endl;
				return -1;
			}
			catch (std::invalid_argument ex)
			{
				std::cout << "Error Parsing Body Data!" << std::endl << ex.what() << std::endl;
				return -1;
			}
		}


		return 1; 

	}
	int Master_sys::Loop()
	{
		while (true) // replace true with user controlled argument
		{
			
			simticks++;
			
			World.Process(time_step, time_mult);

			//Renderer.Process


		}
	}


