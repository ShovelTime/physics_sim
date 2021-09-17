#pragma once
#include <iostream>
#include <vector>
#include "Bodies.h"

// WORLD SUBSYSTEM
// this class initializes and manages the physics sandbox.
// 0.0.0 should be the barycenter of the entire system.
class World_subsys
{

	std::vector<Body> EntityList; // list of all physical entities in the World;
	//camera Camera; 
public:
	World_subsys()
	{


	}
	int Create()
	{

	}
};