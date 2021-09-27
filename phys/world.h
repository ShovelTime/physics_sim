#pragma once
#include <iostream>
#include <vector>
#include "Bodies.h"
#include <nlohmann/json.hpp>
#include <ctime>

// WORLD SUBSYSTEM
// this class holds the bodies and any relevant data for the physics simulation.
// 0.0.0 should be the barycenter of the entire system.
class World_subsys
{
	std::string WorldName; 
	std::vector<Body> EntityList; // list of all physical entities in the World;
	//camera Camera; 
public:
	World_subsys();
	int Create_World(std::string name, std::vector<Body> BodyList);
	void Process(float tstep, float tmult);
	std::vector<Body> GetEntities();

};