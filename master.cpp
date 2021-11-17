#define OLC_PGE_APPLICATION
#define OLC_PGEX_GLX3D
#include "olc/olcPixelGameEngine.h"
#include "olc/olcPGEX_Graphics3D.h"
#include "master.h"

using nlohmann::json;
// MASTER SYSTEM
// controls the flow of the entire simulation, and is the main loop of the program.
class Renderer : public olc::PixelGameEngine
{
public:
	
	//const Master_sys& r_master;
	float zoom_lvl = 1.0f; // 1 = roughly to one million(1.00e06) kilometer per pixel. 
	float zoom_scalar = 1.00e+06f;
	//std::vector<Body>(*taskfunc)(std::vector<Body>);
	std::shared_future<std::vector<Body>>& r_bodyfuture;
	std::packaged_task<std::vector<Body>()>& r_bodytask;
	olc::vf2d center; //center of screen, starts the "0,0,0" point
	std::vector<Body> position_dat;


	int resolution_x = 512; //placeholder
	int resolution_y = 480; //placeholder
	Renderer(std::shared_future<std::vector<Body>>& r_future, std::packaged_task<std::vector<Body>()>& r_task) : r_bodyfuture(r_future), r_bodytask(r_task)
	{
		position_dat = r_bodyfuture.get();
		std::vector<Body>* pos_ptr = &position_dat;
		RECT rwindow;
		const HWND hwindow = GetDesktopWindow();
		GetWindowRect(hwindow, &rwindow);
		resolution_x = rwindow.right;
		resolution_y = rwindow.bottom;
		sAppName = "Physics Sim";
	}
	bool OnUserCreate() override
	{

		center = {(float)ScreenWidth() / 2, (float)ScreenHeight() / 2 };
		return true;
	}
	bool OnUserUpdate(float fElapsedTime) override
	{
		// Erase previous frame
		Clear(olc::DARK_BLUE);
		if (r_bodyfuture.wait_for(std::chrono::nanoseconds(0)) == std::future_status::ready)
		{
			position_dat = r_bodyfuture.get();
		}

		for (int iter = 0; iter < position_dat.size(); iter++)
		{
			float iter_x = position_dat[iter].position[0] / zoom_scalar;
			float iter_y = position_dat[iter].position[1] / zoom_scalar;
			if (fabs(iter_x) >= center.x || fabs(iter_y) >= center.y) // check if coordinates would be rendered outside of the window screen.
			{
				continue; // dont render it lmao
			}
			iter_x += center.x;
			iter_y += center.y;
			olc::vf2d pos = { iter_x, iter_y };
			float dist = phys::get_distance_num({ iter_x,iter_y,0 }, { 0,0,0 });
			Draw(pos, olc::GREEN);
			DrawCircle(center, dist, olc::YELLOW);
		}
		return true;
	}
public:
	int Init()
	{
		if (Construct(resolution_x, resolution_y, 2, 2))
		{
			Start();
		}
		return 0;
	}
	std::vector<Body> Get_Data(std::vector<Body> result)
	{

		return result;
			
		
	}

};
/*
	World_subsysWorld;
	float timemult = 1.0f; // used to slow down or speed up simulation, 1.0 is real-time.
	float timestep = 0.02f; //real-time that should pass per tick. The tick rate should preferrable be synced to this time.
	float tickrate = timestep; // starts at timestep, can be slowed down if calculation takes too long.
	*/
	
	Master_sys::Master_sys() 
	{
	}
	void Parse_Date(std::string date) 
	{
	}

	void Init_Renderer(Renderer renderer)
	{
		renderer.Init();
	}
	void Master_sys::Init(std::filesystem::path path, std::string file)
	{
		if (!Load_World_Data(path, file)){

		}

		World_subsys& world = World;
		std::packaged_task<std::vector<Body>()> r_bodytask([world]()
			{
				return world.Get_Entities();
			});
		std::shared_future<std::vector<Body>> r_taskfuture = r_bodytask.get_future();
		
		Renderer renderer(r_taskfuture, r_bodytask);

		//std::thread r_thread(Init_Renderer, renderer);//r_bodytask, r_taskfuture);
		//r_thread.detach();
		worldloaded = true;

		std::cout << "Init Complete" << std::endl;
		std::cout << phys::get_distance_num(World.Get_Entities()[0].position, World.Get_Entities()[2].position) << " km" << std::endl;
		std::cout << phys::get_distance_num(World.Get_Entities()[4].position, World.Get_Entities()[3].position) << " km" << std::endl;
		Loop(r_taskfuture, r_bodytask);
		
	}
	int Master_sys::Load_World_Data(std::filesystem::path path, std::string file)
	{
		std::ifstream input_stream(path / file);
		if(input_stream.is_open())
		{
			try {
				json jparser = nlohmann::json{}.parse(input_stream);
				std::string name = jparser.at("World").at(0).at("name").get<std::string>();
				json bodyparser = jparser.at("World").at(0).at("Bodies");
				std::string date = jparser.at("World").at(0).at("date").get<std::string>();
				Parse_Date(date);
				std::vector<Body> Bodies;

				
				for (int i = 0; i < bodyparser.size(); ++i) //Parse Body data into Objects
				{
					Bodies.push_back(
						Body(
						bodyparser.at(i).at("name").get<std::string>(),
						bodyparser.at(i).at("radius").get<float>(),
						bodyparser.at(i).at("mass").get<double>(),
						bodyparser.at(i).at("location").get<std::vector<double>>(),
						bodyparser.at(i).at("velocity").get<std::vector<double>>()
						)
					);
					Body res = Bodies.back();
					if (res.mass < 0.0 || res.radius < 0.0) 
					{
						throw std::invalid_argument("Body nr " + std::to_string((Bodies.size() - 1)) + "; Mass or Radius should not be Negative!\n Name: "+ Bodies.back().name +"\n Mass: " + std::to_string(Bodies.back().mass) + "\n Radius: " + std::to_string(Bodies.back().radius));
					}
					if (res.velocity.size() != 3 || res.position.size() != 3)
					{
						throw std::invalid_argument("Body nr " + std::to_string((Bodies.size() - 1)) + "; Position or velocity vector has wrong size! " + Bodies.back().name + "\n position size: " + std::to_string(Bodies.back().position.size()) + "\n velocity size: " + std::to_string(Bodies.back().velocity.size()));
					}
				}
				if (World.Create_World(name, Bodies)) {
					return 1;
				}
				else
				{
					throw E_NOTIMPL; // replace this ex with something else
				}


				std::cout << "yup" << std::endl;
			
			}
			catch (nlohmann::json::exception ex)
			{
				std::cout << "Error Parsing Json!" << std::endl << ex.what() << std::endl;
				return -1;
			}
			catch (std::invalid_argument ex)
			{
				std::cout << "Error Parsing Body Data!" << std::endl << ex.what() << std::endl;
				return -1;
			}
		}


		return 1; 

	}
	int Master_sys::Loop(std::shared_future<std::vector<Body>>& r_future, std::packaged_task<std::vector<Body>()>& r_task)
	{
		auto startime = std::chrono::high_resolution_clock::now();
		std::cout << "Simulating..." << std::endl;
		while (simticks < 98928) // replace true with user or simulation controlled argument
		{
 			simticks++;
			
			World.Process(time_step, time_mult);
			try
			{
				r_task.reset();
				r_future = r_task.get_future();
				r_task();
			}
			catch (std::exception e)
			{
				std::cout << e.what();
			}


		}
		auto EntList = World.Get_Entities();
		for (int i = 0; i < EntList.size(); i++)
		{
			std::cout << EntList[i].name << "'s location:" << std::endl;
			for (int iter = 0; iter < 3; iter++)
			{
				std::cout << EntList[i].position[iter] << std::endl;
			}
			std::cout << phys::get_distance_num(EntList[i].position, EntList[0].position) << std::endl;
			std::cout << "velocity: " << std::endl;
			for (int iter = 0; iter < 3; iter++)
			{
				std::cout << EntList[i].velocity[iter] << std::endl;
			}
			std::cout << std::endl << std::endl;
		}

		std::cout << phys::get_distance_num(EntList[4].position, EntList[3].position) << std::endl;
		auto stoptime = std::chrono::high_resolution_clock::now();
		std::cout << std::chrono::duration_cast<std::chrono::microseconds>(stoptime - startime).count() << " microseconds" << std::endl;
		return 1;
	}


