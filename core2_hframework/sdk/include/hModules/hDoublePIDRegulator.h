/**
 * Copyright (c) 2013-2017 Husarion Sp. z o.o.
 * Distributed under the MIT license.
 * For full terms see the file LICENSE.md.
 */

#ifndef H_DOUBLE_PID_REGULATOR_H
#define H_DOUBLE_PID_REGULATOR_H

#include "hRegulator.h"
#include "hDoublePID.h"

namespace hFramework
{
class hDoublePIDRegulator : public virtual hRegulator, public hDoublePID
{
private:
	uint8_t stableCounter;

public:
	hDoublePIDRegulator();
	virtual ~hDoublePIDRegulator() { }

	int32_t stableRange;
	int stableTimes;

	regFuncState regFunct(int32_t encoderNow, int32_t encoderTarget, uint16_t power, int16_t& motorOut);
};
}

#endif