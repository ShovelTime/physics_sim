#include <iostream>
#include <vector>
//#include "../const.h"

// EVERY RELEVANT DATA SHOULD BE FORMATTED IN THEIR SI UNITS WHENEVER POSSIBLE.
// this class represents any object that can be affected or interacted with by the physics engine
class Body
{
	// ints should be changed to floating point types for better accuracy
	char name[128];
	float radius; //m
	double mass; //kg
	std::vector<double> position;// 3d coordinates relative to 0,0,0 <m>
	std::vector<double> velocity;// movement in 3d space relative to the static frame of reference of the world. <m/s>
	
	




};