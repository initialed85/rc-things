/*
 * This file is part of hFramework
 *
 * Copyright (C) 2014 Husarion Sp. z o.o. <husarion.com> -  All Rights Reserved
 *
 * Unauthorized copying of this file and the hFramework library, via any medium
 * is strictly prohibited.
 * Proprietary and confidential.
*/

#ifndef H_LED_H
#define H_LED_H

#include "hTypes.h"
#include "hGPIO.h"

namespace hFramework
{
namespace stm32
{
	
/**
 * @brief Controls the on-board LED
 */
class hLEDClass
{
	friend class hSystem;

public:
	hLEDClass(uint8_t nr);

	void on(); //!< Turn LED on.
	void off(); //!< Turn LED off.
	void toggle(); //!< Turn LED off, if it is on, or turn LED on, if it is off.
	void set(bool enabled);

private:
	void init();

	hLEDClass(const hLEDClass&) = delete;

	hGPIO gpio;
};

}
}

#endif /* H_LED_H */
