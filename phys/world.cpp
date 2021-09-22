//ZA WARUDO
#include "world.h"
#include <iostream>
#include <vector>
#include "Bodies.h"
#include <nlohmann/json.hpp>




World_subsys::World_subsys() 
{

}
int World_subsys::Create_World(std::string name, nlohmann::json BodyList) 
{
	WorldName = name;
	std::cout << BodyList.dump();
	return 1;


}


