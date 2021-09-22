#pragma once
#include <iostream>
#include <vector>
#include "Bodies.h"
#include <nlohmann/json.hpp>

// WORLD SUBSYSTEM
// this class initializes and manages the physics sandbox.
// 0.0.0 should be the barycenter of the entire system.
class World_subsys
{
	std::string WorldName; 
	std::vector<Body> EntityList; // list of all physical entities in the World;
	//camera Camera; 
public:
	World_subsys();
	int Create_World(std::string name, nlohmann::json BodyList);
};