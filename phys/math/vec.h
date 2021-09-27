#pragma once

#include <vector>
#include <iostream>
#include <cmath>

namespace vec {
	std::vector<double> vec_add(std::vector<double> vec_1, std::vector<double> vec_2)
	{
		for (int i = 0; i < vec_1.size(); i++)
		{
			vec_1[i] += vec_2[i];
		}
		return vec_1;
	}
	std::vector<double> vec_substract(std::vector<double> vec_1, std::vector<double> vec_2)
	{
		for (int i = 0; i < vec_1.size(); i++)
		{
			vec_1[i] -= vec_2[i];
		}
		return vec_1;

	}
	std::vector<double> vec3_normalise(std::vector<double> vec_in)
	{
		double vec_len = (1 / sqrt(pow(vec_in[0], 2) + pow(vec_in[1], 2) + pow(vec_in[2], 2))); // is there a better way of doing this? though this avoids having to do a division for every iterations in the for loop.
		for (int i = 0; i < vec_in.size(); i++)
		{
			vec_in[i] = vec_in[i] * vec_len;
		}

		return vec_in;
	}
	std::vector<double> vec_negate(std::vector<double> vec_in)
	{
		for (int i = 0; i < vec_in.size(); i++)
		{
			vec_in[i] *= -1;

		}
		return vec_in;
	}
	std::vector<double> vec_mult(std::vector<double> vec_1, std::vector<double> vec_2)
	{
		for (int i = 0; i < vec_1.size(); i++)
		{
			vec_1[i] *= vec_2[i];
		}
		return vec_1;
	}

	std::vector<double> vec_divide(std::vector<double> vec_1, std::vector<double> vec_2) 
	{
		for (int i = 0; i < vec_1.size(); i++)
		{
			vec_1[i] /= vec_2[i];
		}
		return vec_1;

	}

	std::vector<double> vec_scalar_add(std::vector<double> vec_in, double scalar)
	{
		for (int i = 0; i < vec_in.size(); i++)
		{
			vec_in[i] += scalar;
		}
		return vec_in;
	}
	std::vector<double> vec_scalar_substract(std::vector<double> vec_in, double scalar)
	{
		for (int i = 0; i < vec_in.size(); i++)
		{
			vec_in[i] -= scalar;
		}
		return vec_in;
	}
	std::vector<double> vec_scalar_mult(std::vector<double> vec_in, double scalar)
	{
		for (int i = 0; i < vec_in.size(); i++)
		{
			vec_in[i] *= scalar;
		}
		return vec_in;
	}
	std::vector<double> vec_scalar_divide(std::vector<double> vec_in, double scalar)
	{
		for (int i = 0; i < vec_in.size(); i++)
		{
			vec_in[i] /= scalar;
		}
		return vec_in;
	}

}