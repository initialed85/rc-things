/*
 * This file is part of hFramework
 *
 * Copyright (C) 2014 Husarion Sp. z o.o. <husarion.com> -  All Rights Reserved
 *
 * Unauthorized copying of this file and the hFramework library, via any medium
 * is strictly prohibited.
 * Proprietary and confidential.
*/

#ifndef __HI2C_H__
#define __HI2C_H__

#include <stdint.h>

#include <hTypes.h>
#include <hGPIO.h>
#include <II2C.h>

namespace hFramework
{
namespace stm32
{

enum hI2C_ID
{
#if BOARD(ROBOCORE) || BOARD(CORE2)
	hI2C_ID_SENS1,
	hI2C_ID_SENS2,
	hI2C_ID_EXT,
#elif BOARD(CORE2MINI)
	hI2C_ID_SENS1,
#else
#  error no board
#endif
	hI2C_ID_INVALID,
};

using namespace interfaces;

/**
 * @brief Implementation of on-board I2C interface.
 */
class hI2C : public II2C
{
	friend class hSensor_i2c;
	friend class hExtClass;

public:
	hGPIO pinScl, pinSda;

	hI2C(hI2C_ID no);

	void setDataRate(uint32_t bps);
	bool write(uint8_t addr, uint8_t* data, uint32_t len);
	bool read(uint8_t addr, uint8_t* data, uint32_t len);
	bool rw(uint8_t addr, uint8_t* dataTx, uint32_t lengthTx, uint8_t* dataRx, uint32_t lengthRx);

	void selectGPIO();
	void selectI2C();

private:
	int i2cNum;

	void init(uint32_t bps = 10000);
	void deinit();

	hI2C(const hI2C&) = delete;
};

}
}

#endif
