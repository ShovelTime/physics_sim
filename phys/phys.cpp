#include "phys.h"

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
	double get_distance_num(std::vector<double> in)
	{
		
		return sqrt(pow(in[0], 2) + pow(in[1], 2) + pow(in[2], 2));
	}
	std::vector<double> get_direction(std::vector<double> loca, std::vector<double> locb)
	{
		std::vector<double> dist = get_distance_vec(loca, locb);
		return vec::vec3_normalise(dist);
	}
	std::vector<double> get_direction(std::vector<double> in)
	{
		return vec::vec3_normalise(in);
	}
	std::vector<double> get_acceleration_vec(Body orig, Body tgt)
	{
		double first_arg = gravity_constant * tgt.mass;
		auto dist = get_distance_vec(tgt.position, orig.position);
		double mag = get_distance_num(dist);

		mag = mag * mag * mag;
		return vec::vec_scalar_divide(vec::vec_scalar_mult(dist, first_arg), mag);

	}
	std::vector<double> get_acceleration_vec(std::vector<double> pos, Body tgt)
	{

		double first_arg = gravity_constant * tgt.mass;
		auto dist = get_distance_vec(tgt.position, pos);
		double mag = get_distance_num(dist);

		mag = mag * mag * mag;
		return vec::vec_scalar_divide(vec::vec_scalar_mult(dist, first_arg), mag);

	}
}