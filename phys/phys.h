#pragma once
#include <cmath>
#include <vector>
#include "math/vec.h"
#include "math/const.h"
namespace phys
{

	std::vector<double> get_distance_vec(std::vector<double> loca, std::vector<double> locb)
	{
		return vec::vec_substract(loca, locb);
	}
	double get_distance_num(std::vector<double> loca, std::vector<double> locb)
	{
		auto dist = vec::vec_substract(loca, locb);
		return sqrt(pow(dist[0], 2) + pow(dist[1], 2) + pow(dist[2], 2));
	}


	std::vector<double> get_direction(std::vector<double> loca, std::vector<double> locb)
	{
		std::vector<double> dist = get_distance_vec(loca,locb);
		return vec::vec3_normalise(dist);
	}
	std::vector<double> get_direction(std::vector<double> in)
	{
		return vec::vec3_normalise(in);
	}
	std::vector<double> get_acceleration_vec(Body orig, Body tgt)
	{
		double first_arg = gravity_constant * tgt.mass;
		auto dist = vec::vec_substract(tgt.position, orig.position);
		auto unit_vec = get_direction(dist);

		for (int i = 0; i < 3; i++)
		{
			unit_vec[i] = unit_vec[i] * unit_vec[i] * unit_vec[i];
		}
		return vec::vec_scalar_mult(vec::vec_divide(dist, unit_vec), first_arg);

	}


}

