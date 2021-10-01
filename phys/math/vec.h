#ifndef VEC_H
#define VEC_H

#include <vector>
#include <iostream>
#include <cmath>

std::vector<double> operator+(const std::vector<double>& vec_1, const std::vector<double>& vec_2);

namespace vec {

	std::vector<double> vec_add(std::vector<double> vec_1, std::vector<double> vec_2);

	std::vector<double> vec_substract(std::vector<double> vec_1, std::vector<double> vec_2);

	std::vector<double> vec3_normalise(std::vector<double> vec_in);

	std::vector<double> vec_negate(std::vector<double> vec_in);

	std::vector<double> vec_mult(std::vector<double> vec_1, std::vector<double> vec_2);

	std::vector<double> vec_divide(std::vector<double> vec_1, std::vector<double> vec_2);

	std::vector<double> vec_scalar_add(std::vector<double> vec_in, double scalar);

	std::vector<double> vec_scalar_substract(std::vector<double> vec_in, double scalar);

	std::vector<double> vec_scalar_mult(std::vector<double> vec_in, double scalar);

	std::vector<double> vec_scalar_divide(std::vector<double> vec_in, double scalar);


}
#endif