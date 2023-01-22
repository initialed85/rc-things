/*
 * This file is part of hFramework
 *
 * Copyright (C) 2014 Husarion Sp. z o.o. <husarion.com> -  All Rights Reserved
 *
 * Unauthorized copying of this file and the hFramework library, via any medium
 * is strictly prohibited.
 * Proprietary and confidential.
*/

#ifndef __ISPI_H__
#define __ISPI_H__

#include <stdint.h>

namespace hFramework
{

namespace interfaces
{

/**
 * @brief Interface for SPI.
 */
class ISPI
{
public:
	virtual bool write(void* data, uint32_t len) = 0;
	virtual bool read(void* data, uint32_t len) = 0;
	virtual bool rw(void* dataTx, void* dataRx, uint32_t len) = 0;
};

}

}

#endif
