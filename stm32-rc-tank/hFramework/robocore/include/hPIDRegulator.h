/*
 * This file is part of hFramework
 *
 * Copyright (C) 2014 Husarion Sp. z o.o. <husarion.com> -  All Rights Reserved
 *
 * Unauthorized copying of this file and the hFramework library, via any medium
 * is strictly prohibited.
 * Proprietary and confidential.
*/

#ifndef __H_PID_REGULATOR_H__
#define __H_PID_REGULATOR_H__

#include "hRegulator.h"
#include "hPID.h"

using namespace hFramework;

class hPIDRegulator : public virtual hRegulator, public hPID
{
private:
	uint8_t stableCounter;

public:
	hPIDRegulator();
	virtual ~hPIDRegulator() { }

	int32_t stableRange;
	int stableTimes;

	regFuncState regFunct(int32_t encoderNow, int32_t encoderTarget, uint16_t power, int16_t& motorOut);
};

#endif
