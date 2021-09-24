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


