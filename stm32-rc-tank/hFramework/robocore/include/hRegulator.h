/*
 * This file is part of hFramework
 *
 * Copyright (C) 2014 Husarion Sp. z o.o. <husarion.com> -  All Rights Reserved
 *
 * Unauthorized copying of this file and the hFramework library, via any medium
 * is strictly prohibited.
 * Proprietary and confidential.
*/

#ifndef __H_REGULATOR__
#define __H_REGULATOR__

#include "hTypes.h"

namespace hFramework
{

enum regFuncState
{
	REG_IN_PROGRESS = 0,
	REG_FINISH_BREAK,
	REG_FINISH_CONTINUE
};

/**
* @brief Virtual regulator class that provides interface for different types of motor regulators (see hMotor).
*/
class hRegulator
{
public:
	virtual ~hRegulator() { }

	//! @cond
	uint16_t dtMs;
	//! @endcond

//protected:
	/**
	 * @brief Function called every time regulation is needed.
	 * @param encoderNow current encoder value
	 * @param encoderTarget desired encoder value (setpoint)
	 * @param dtMs current time delta in ms
	 * @param power current power
	 * @param motorOut motor output power
	 * @return Regulator state.
	 */
	virtual regFuncState regFunct(int32_t encoderNow, int32_t encoderTarget, uint16_t power, int16_t& motorOut) = 0;
};

}
#endif //__H_REGULATOR__
