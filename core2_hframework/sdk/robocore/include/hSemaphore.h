/*
 * This file is part of hFramework
 *
 * Copyright (C) 2014 Husarion Sp. z o.o. <husarion.com> -  All Rights Reserved
 *
 * Unauthorized copying of this file and the hFramework library, via any medium
 * is strictly prohibited.
 * Proprietary and confidential.
*/

#ifndef __HSEMAPHORE_H__
#define __HSEMAPHORE_H__

#include <hTypes.h>

namespace hFramework
{

/**
 * @brief Semaphore class.
 *
 * Semaphores are used by RTOS (Real-Time Operating System) to provide task synchronization.
 */
class hSemaphore
{
public:
	hSemaphore(); //!< Initialize semaphore.
	~hSemaphore(); //!< Destroy semaphore.

	bool give();
	bool take(uint32_t timeout = INFINITE);

private:
	class hSemaphorePimpl;
	hSemaphorePimpl* impl;
	
	hSemaphore(const hSemaphore&) = delete;
};

}

#endif /* __HSEMAPHORE_H__ */
