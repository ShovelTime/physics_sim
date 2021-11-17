//ZA WARUDO
#include "world.h"


/*
std::string WorldName;
std::vector<Body> EntityList; // list of all physical entities in the World;
std::vector<phys::uppdtpkg> Pending; // list of updated vectors to apply to entities at the end of Process()
*/

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

		Body& sel = EntityList[iter];
		phys::uppdtpkg currpkg;
		// implement Verlet Vector integration
;
		std::vector<double> accel1 = Calc_Acceleration(iter, lsize);

		currpkg.name = sel.name;

		currpkg.new_pos = sel.position + vec::vec_scalar_mult(sel.velocity, time_increment) + vec::vec_scalar_mult(vec::vec_scalar_mult(accel1, 0.5), (time_increment * time_increment));

		std::vector<double> accel2 = Calc_Acceleration(iter, lsize, currpkg.new_pos);

		currpkg.new_vel = sel.velocity + vec::vec_scalar_mult(accel1 + accel2, (time_increment * 0.5));

		Pending.push_back(currpkg);
		
	}

	//Update position and velocities
	if (Pending.size() != EntityList.size())
	{
		throw std::invalid_argument("Pending vector is not of similar size as the EntityList!");
	}
	for (int iter = 0; iter < lsize; iter++)
	{
		EntityList[iter].position = Pending[iter].new_pos;
		EntityList[iter].velocity = Pending[iter].new_vel;
	}
	Pending.clear();

	
}
const std::vector<Body> World_subsys::Get_Entities() const
{
	return EntityList;
}
std::string World_subsys::Get_Name() 
{
	return WorldName;
}
std::vector<double> World_subsys::Calc_Acceleration(int iter, int lsize)
{
	std::vector<double> accel;

	for (int iter2 = 0; iter2 < lsize; iter2++)
	{
		if (iter2 == iter) // the body shouldnt affect itself
		{
			continue;

		}
		accel = phys::get_acceleration_vec(EntityList[iter], EntityList[iter2]);

	}
	return accel;

}
std::vector<double> World_subsys::Calc_Acceleration(int iter, int lsize, std::vector<double> pos_2)
{
	std::vector<double> accel{ 0, 0, 0 };

	for (int iter2 = 0; iter2 < lsize; iter2++)
	{
		if (iter2 == iter) // the body shouldnt affect itself
		{
			continue;

		}
		accel = vec::vec_add(accel, phys::get_acceleration_vec(pos_2, EntityList[iter2]));

	}
	return accel;

}