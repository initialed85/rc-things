/*
 * This file is part of hFramework
 *
 * Copyright (C) 2014 Husarion Sp. z o.o. <husarion.com> -  All Rights Reserved
 *
 * Unauthorized copying of this file and the hFramework library, via any medium
 * is strictly prohibited.
 * Proprietary and confidential.
*/

#ifndef __HMUTEX_H__
#define __HMUTEX_H__

#include <hTypes.h>

namespace hFramework
{

/**
 * @brief Mutex class.
 *
 * Mutexes are used by Operating System to provide task synchronization.
 */
class hMutex
{
public:
	enum EType { Normal, Recursive };

	hMutex(EType type = Normal); //!< Initialize mutex.
	virtual ~hMutex(); //!< Destroy mutex.

	/**
	 * @brief Try to take a mutex and block a task until another task makes the mutex available.
	 * @param timeout amount of time to wait for a mutex, in ms. INFINITE to wait indefinitely.
	 * @return true if Mutex has been successfully taken before timeout, false otherwise.
	 */
	bool lock(uint32_t timeout = INFINITE);
	bool take(uint32_t timeout = INFINITE) { return lock(timeout); }

	/**
	 * @brief Give mutex to blocked tasks.
	 *
	 * If mutex isn't necessary it can be given to blocked tasks.
	 * @return true if Mutex is ready to be taken.
	 */
	bool unlock();
	bool give() { return unlock(); }

private:
	class hMutexPimpl;
	hMutexPimpl* impl;

	hMutex(const hMutex&) = delete;

	void* getMutex();

	friend class hCondVar;
};

/**
 * @brief Recursive mutex class.
 *
 * Mutexes are used by RTOS (Real-Time Operating System) to provide task synchronization.
 */
class hRecursiveMutex : public hMutex
{
public:
	hRecursiveMutex() : hMutex(Recursive) { } //!< Initialize recursive mutex.
	~hRecursiveMutex() { } //!< Destroy mutex.
};

/**
 * @brief Automatically takes a mutex and releases it when the guard goes out of scope.
 */
class hMutexGuard
{
public:
	hMutexGuard(hMutex& mutex);
	~hMutexGuard();

private:
	hMutex& mutex;

	hMutexGuard(const hMutexGuard&) = delete;
};

}

#endif /* __HMUTEX_H__ */
