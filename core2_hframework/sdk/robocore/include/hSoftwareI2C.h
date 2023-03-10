/*
 * This file is part of hFramework
 *
 * Copyright (C) 2014 Husarion Sp. z o.o. <husarion.com> -  All Rights Reserved
 *
 * Unauthorized copying of this file and the hFramework library, via any medium
 * is strictly prohibited.
 * Proprietary and confidential.
*/

#ifndef __HSOFTWAREI2C_H__
#define __HSOFTWAREI2C_H__

#include <stdint.h>

#include <hTypes.h>
#include <IGPIO.h>
#include <II2C.h>

namespace hFramework
{

namespace interfaces
{
class ISensor;
}

using namespace interfaces;

/**
 * @brief Implementation of software I2C interface.
 */
class hSoftwareI2C : public II2C
{
public:
	hSoftwareI2C(IGPIO& sda, IGPIO& scl);
	hSoftwareI2C(ISensor& sens);

	void init(uint32_t bps = 10000);
	void deinit();

	void setDataRate(uint32_t bps);
	bool write(uint8_t addr, uint8_t* data, uint32_t len);
	bool read(uint8_t addr, uint8_t* data, uint32_t len);
	bool rw(uint8_t addr, uint8_t* dataTx, uint32_t lengthTx, uint8_t* dataRx, uint32_t lengthRx);

private:
	IGPIO& pinSDA;
	IGPIO& pinSCL;
	int delayVal;

	void delay();
	uint8_t writeByte(uint8_t b);
	uint8_t readByte(int ack);
	void sendStart();
	void sendStop();

	hSoftwareI2C(const hSoftwareI2C&) = delete;
};

}

#endif
