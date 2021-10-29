#ifndef WORLD_H
#define WORLD_H


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
	std::vector<double> Calc_Acceleration(int iter, int lsize, std::vector<double> pos_2);
public:
	World_subsys();
	int Create_World(std::string name, std::vector<Body> BodyList);
	void Process(float tstep, float tmult);
	const std::vector<Body> Get_Entities() const;
	std::string Get_Name();
};
#endif // !WORLD_H