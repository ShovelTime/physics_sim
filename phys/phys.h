#ifndef PHYS_H
#define PHYS_H


#include <cmath>
#include <vector>
#include "math/vec.h"
#include "math/const.h"
#include "bodies.h"
namespace phys
{

	struct uppdtpkg // container holding the new velocity and position of a body to be applied after the physics calculation stage.
	{
		std::string name;
		std::vector<double> new_pos;
		std::vector<double> new_vel;
	};

	std::vector<double> get_distance_vec(std::vector<double> loca, std::vector<double> locb);

	double get_distance_num(std::vector<double> loca, std::vector<double> locb);

	std::vector<double> get_direction(std::vector<double> loca, std::vector<double> locb);

	std::vector<double> get_direction(std::vector<double> in);

	std::vector<double> get_acceleration_vec(Body orig, Body tgt);

	std::vector<double> get_acceleration_vec(std::vector<double> pos, Body tgt);

}
#endif
