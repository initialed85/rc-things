/*
 * This file is part of hFramework
 *
 * Copyright (C) 2014 Husarion Sp. z o.o. <husarion.com> -  All Rights Reserved
 *
 * Unauthorized copying of this file and the hFramework library, via any medium
 * is strictly prohibited.
 * Proprietary and confidential.
*/

#ifndef __IMUTEX_H__
#define __IMUTEX_H__

#include <hTypes.h>

namespace hFramework
{
enum class MutexType { Mutex, RecursiveMutex, Semaphore };

namespace interfaces
{
	
/**
 * @brief Mutex class.
 *
 * Mutexes are used by Operating System to provide task synchronization.
 */
class IMutex
{
public:
	virtual ~IMutex() { }; //!< Destroy mutex.

	/**
	 * @brief Give mutex to blocked tasks.
	 *
	 * If mutex isn't necessary it can be given to blocked tasks.
	 * @return true if Mutex is ready to be taken.
	 */
	virtual bool give() = 0;
	bool unlock() { return give(); }

	/**
	 * @brief Try to take a mutex and block a task until another task makes the mutex available.
	 * @param timeout amount of time to wait for a mutex, in ms. INFINITE to wait indefinitely.
	 * @return true if Mutex has been successfully taken before timeout, false otherwise.
	 */
	virtual bool take(uint32_t timeout = INFINITE) = 0;
	bool lock(uint32_t timeout = INFINITE) { return take(timeout); }
};

} // namespace interfaces

using namespace interfaces;

/**
 * @brief Automatically takes a mutex and releases it when the guard goes out of scope.
 */
class hMutexGuard
{
public:
	hMutexGuard(IMutex& mutex) : mutex(mutex) { mutex.lock(); }
	~hMutexGuard() { mutex.unlock(); }

private:
	IMutex& mutex;

	hMutexGuard(const hMutexGuard&) = delete;
};

} // namespace hFramework

#endif /* __IMUTEX_H__ */
