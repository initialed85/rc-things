/*
 * This file is part of hFramework
 *
 * Copyright (C) 2014 Husarion Sp. z o.o. <husarion.com> -  All Rights Reserved
 *
 * Unauthorized copying of this file and the hFramework library, via any medium
 * is strictly prohibited.
 * Proprietary and confidential.
*/

#ifndef __HGENERICQUEUE_H__
#define __HGENERICQUEUE_H__

#include "hTypes.h"
namespace hFramework
{
/**
 * @brief Provides generic, byte oriented FIFO and LIFO queues.
 */
class hGenericQueue
{
public:
	hGenericQueue(uint32_t maxSize, uint32_t itemSize);

	~hGenericQueue();

	bool put(const void* item, uint32_t timeout = 0xffffffff);
	
	bool putFront(const void* item, uint32_t timeout = 0xffffffff);

	bool putFromISR(const void* item, long* xHigherPriorityTaskWoken);

	bool get(void* item, uint32_t timeout = 0xffffffff);

	bool peek(void* item, uint32_t timeout = 0xffffffff);

	uint32_t size();

	uint32_t freeSpace();

	bool clear();

private:
	class hGenericQueuePimpl;
	hGenericQueuePimpl *impl;
	
	hGenericQueue(const hGenericQueue&) = delete;
};

}
#endif
