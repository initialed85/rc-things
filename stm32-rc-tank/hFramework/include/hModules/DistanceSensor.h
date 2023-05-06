#ifndef __DISTANCESENSOR_H__
#define __DISTANCESENSOR_H__

#include <hFramework.h>

namespace hModules
{

class DistanceSensorPimpl;

/**
 * @brief Controls HC-SR04 ultrasonic distance sensor
 */
class DistanceSensor
{
public:
	DistanceSensor(ISensor& sens);

	void init(); //!< Initialize distance sensor

	int16_t getDistance();

private:
	DistanceSensorPimpl* impl;

	DistanceSensor(const DistanceSensor&) = delete;
};

}

#endif
