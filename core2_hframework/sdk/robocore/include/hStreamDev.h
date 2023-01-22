/*
 * This file is part of hFramework
 *
 * Copyright (C) 2014 Husarion Sp. z o.o. <husarion.com> -  All Rights Reserved
 *
 * Unauthorized copying of this file and the hFramework library, via any medium
 * is strictly prohibited.
 * Proprietary and confidential.
*/

#ifndef H_STREAM_DEV
#define H_STREAM_DEV

#include "hTypes.h"
#include <stdarg.h>

namespace hFramework
{

/**
 * @brief Stream device interface.
 */
class hStreamDev
{
public:
	/**
	 * @brief Write data to the underlying device
	 * @param data data to be written
	 * @param len number of bytes to write
	 * @param timeout maximum timeout of the operation
	 * @return Number of written bytes.
	 */
	virtual int write(const void* data, int len, uint32_t timeout = 0xffffffff) = 0;

	/**
	 * @brief Read data from the underlying device
	 * @param data data to be read
	 * @param len number of bytes to read
	 * @param timeout maximum timeout of the operation
	 * @return Number of read bytes.
	 */
	virtual int read(void* data, int len, uint32_t timeout = 0xffffffff) = 0;

	int readAll(void* data, int len, uint32_t timeout = 0xffffffff);
};

}
#endif //H_STREAM_DEV
