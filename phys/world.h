#pragma once
#include <iostream>
#include <vector>
#include "Bodies.h"
#include <nlohmann/json.hpp>
#include <ctime>
#include "phys.h"

// WORLD SUBSYSTEM
// this class holds the bodies and any relevant data for the physics simulation.
// 0.0.0 should be the barycenter of the entire system.
class World_subsys
{
	std::string WorldName; 
	std::vector<Body> EntityList; // list of all physical entities in the World;
	std::vector<phys::uppdtpkg> Pending; // list of updated vectors to apply to entities at the end of Process()
	//camera Camera; 
	std::vector<double> Calc_Acceleration(int iter,int lsize);
public:
	World_subsys();
	int Create_World(std::string name, std::vector<Body> BodyList);
	void Process(float tstep, float tmult);
	std::vector<Body> Get_Entities();
	std::string Get_Name();
};