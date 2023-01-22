/*
 * This file is part of hFramework
 *
 * Copyright (C) 2014 Husarion Sp. z o.o. <husarion.com> -  All Rights Reserved
 *
 * Unauthorized copying of this file and the hFramework library, via any medium
 * is strictly prohibited.
 * Proprietary and confidential.
*/

#ifndef __HQUEUE_H__
#define __HQUEUE_H__

#include "hGenericQueue.h"
#include "hSystem.h"

namespace hFramework
{

/**
 * @brief Provides type oriented FIFO and LIFO queues.
 *
 * Queues are used by RTOS (Real-Time Operating System) to exchange data between different tasks.
 * @tparam T - Element type
 */
template<typename T>
class hQueue
{
public:
	/**
	 * Initialize queue.
	 * @param length number of element in the queue
	 */
	hQueue(uint32_t length);

	/**
	 * @brief Send a single element to back (FIFO - First-In-First-Out type queue)
	 * @param Item element to be sent to queue
	 * @param timeout amount of time to wait if queue if full.
	 * @return true if element has been successfully sent to queue.
	 */
	bool sendToBack(const T& Item, uint32_t timeout = INFINITE);

	/**
	 * @brief Send a single element to front (LIFO - Last-In-First-Out type queue)
	 * @param Item element to be sent to queue
	 * @param timeout amount of time to wait if queue if full.
	 * @return true if element has been successfully sent to queue.
	 */
	bool sendToFront(const T& Item, uint32_t timeout = INFINITE);

	/**
	 * @brief Remove all elements from the queue.
	 * @return true if queue is flushed successfully.
	 */
	bool flush();

	/**
	 * @brief Get a single element from the front of a queue.
	 * @param Item place where to store received element.
	 * @param timeout amount of time to wait if queue if empty.
	 * @return true if element has been successfully received from the queue.
	 */
	bool receive(T& Item, uint32_t timeout = INFINITE);

	/**
	 * @brief Copy a single element from the front of a queue. After this operation queue has the same elements inside, that before.
	 * @param Item place where to store copied element.
	 * @param timeout amount of time to wait if queue if empty.
	 * @return true if element has been successfully copied from the queue.
	 */
	bool copyFromFront(T& Item, uint32_t timeout = INFINITE);

	/**
	 * @brief Get number of elements stored inside the queue.
	 * @return number of elements inside the queue.
	 */
	uint32_t getElementCnt();

private:
	hGenericQueue queue;

	hQueue(const hQueue&) = delete;
};


template<typename T>
hQueue<T>::hQueue(uint32_t length)
	: queue(length, sizeof(T))
{
}

template<typename T>
bool hQueue<T>::sendToBack(const T& Item, uint32_t timeout)
{
	return queue.put(&Item, timeout);
}

template<typename T>
bool hQueue<T>::sendToFront(const T& Item, uint32_t timeout)
{
	return queue.putFront(&Item, timeout);
}

template<typename T>
bool hQueue<T>::flush()
{
	return queue.clear();
}

template<typename T>
bool hQueue<T>::receive(T& Item, uint32_t timeout)
{
	return queue.get(&Item, timeout);
}

template<typename T>
bool hQueue<T>::copyFromFront(T& Item, uint32_t timeout)
{
	return queue.peek(&Item, timeout);
}

template<typename T>
uint32_t hQueue<T>::getElementCnt()
{
	return queue.size();
}

}

#endif /* __HQUEUE_H__ */
