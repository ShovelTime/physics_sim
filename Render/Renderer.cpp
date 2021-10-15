#define OLC_PGE_APPLICATION
#include "olc/olcPixelGameEngine.h"
#include "Renderer.h"



Renderer::Renderer()
{
	sAppName = "Physics Sim";
}
bool Renderer::OnUserCreate() 
{
	return true;
}
bool Renderer::OnUserUpdate(float fElapsedTime)
{
	return true;
}
int Renderer::Init()
{
	return 0;
}