/*
 * This file is part of hFramework
 *
 * Copyright (C) 2014 Husarion Sp. z o.o. <husarion.com> -  All Rights Reserved
 *
 * Unauthorized copying of this file and the hFramework library, via any medium
 * is strictly prohibited.
 * Proprietary and confidential.
*/

#ifndef __HCAN_H__
#define __HCAN_H__

#include <hTypes.h>
#include <hGPIO.h>

#ifdef CAN_ENABLED

namespace hFramework
{
namespace stm32
{

/**
 * @brief Provides CAN interface.
 */
class hCAN
{
public:
	hCAN();

	/**
	 * Initialize a CAN interface.
	 */
	void init();

	/**
	 * Send CAN frame through CAN interface. This method blocks
	 * task until frame is sent.
	 * @param frame reference to CAN frame object used for transmit data
	 */
	void sendFrame(CAN_frameTx& frame);

	void enable();
	void disable();

	/**
	 * Read CAN frame from a buffer (if buffer is not empty) or wait for a frame.
	 * @param frame reference to CAN frame object used to store received CAN frames.
	 * @param timeout a timeout value (in milliseconds)
	 * @return true - if frame has been received before timeout, otherwise false.
	 */
	bool waitFrame(CAN_frameRx& frame, uint32_t timeout = INFINITE);

private:
	hCAN(const hCAN&) = delete;

	hGPIO pinEnable;
};

}
}

#endif

#endif /* __HCAN_H__ */
