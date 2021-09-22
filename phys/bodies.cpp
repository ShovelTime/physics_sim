#include "bodies.h"


Body::Body(std::string in_name, float in_radius, double in_mass, std::vector<double> in_position, std::vector<double> in_velocity)
{
	name = in_name;
	radius = in_radius;
	mass = in_mass;
	position = in_position;
	velocity = in_velocity;
}

