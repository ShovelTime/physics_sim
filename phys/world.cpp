//ZA WARUDO
#include "world.h"
#include <iostream>
#include <vector>
#include "Bodies.h"
#include <nlohmann/json.hpp>




World_subsys::World_subsys() 
{

}
int World_subsys::Create_World(std::string name, std::vector<Body> BodyList)
{
	
	WorldName = name;
	EntityList = BodyList;
	return 1;


}
void World_subsys::Process(float tstep, float tmult)
{
	float time_increment = tstep * tmult;
	for (int iter = 0; iter < EntityList.size(); iter++)
	{

	}
}
std::vector<Body> World_subsys::GetEntities()
{
	return EntityList;
}