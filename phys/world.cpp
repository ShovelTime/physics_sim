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

	int lsize = EntityList.size();
	float time_increment = tstep * tmult;
	for (int iter = 0; iter < lsize; iter++)
	{
		Pending.push_back(phys::uppdtpkg{});
		phys::uppdtpkg& currpkg = Pending.back();
		// implement Verlet Vector integration

			std::vector<double> accel = phys::get_acceleration_vec(EntityList[iter], EntityList[iter2]);

			

		}
	}
}
std::vector<Body> World_subsys::Get_Entities()
{
	return EntityList;
}
std::string World_subsys::Get_Name() 
{
	return WorldName;
}
std::vector<double> World_subsys::Calc_Acceleration(int iter)
{
	for (int iter2 = 0; iter2 < lsize; iter2++)
	{
		if (iter2 == iter) // the body shouldnt affect itself
		{
			continue;

		}
}