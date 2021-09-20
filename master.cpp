


// MASTER SUBSYSTEM
// controls the flow of the entire simulation, and is the main loop of the program.

class Master_subsys 
{
	float timemult = 1.0f; // used to slow down or speed up simulation.
	float timestep = 0.02f; //time that should pass per tick. The tick rate should preferrable be synced to this time.
	float tickrate = timestep; // starts at timestep, can be slowed down if calculation takes too long.

public:
	void Init() 
	{

	}
	void Load_World(char* path, char* file)
	{

	}

};

