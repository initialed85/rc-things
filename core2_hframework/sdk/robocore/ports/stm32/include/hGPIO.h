/*
 * This file is part of hFramework
 *
 * Copyright (C) 2014 Husarion Sp. z o.o. <husarion.com> -  All Rights Reserved
 *
 * Unauthorized copying of this file and the hFramework library, via any medium
 * is strictly prohibited.
 * Proprietary and confidential.
*/

#ifndef __HGPIO_H__
#define __HGPIO_H__

#include <stdint.h>

#include <IGPIO.h>

namespace hFramework
{
namespace stm32
{

using namespace interfaces;

/**
 * @brief Implementation of on-board General Purpose Input-Output pin (GPIO).
 */
class hGPIO : public virtual IGPIO
{
protected:
	uint8_t nr;

public:
	hGPIO(uint8_t nr);

	virtual void init();

	void setOut();
	void setOutOD();
	void setOutOD_pu();
	void setOutOD_pd();
	void setIn();
	void setIn_pu();
	void setIn_pd();

	bool read();

	void write(bool value);

	void toggle();
	void reset();
};

/**
 * @brief Implementation of on-board General Purpose Input-Output pin (GPIO) with interrupt capability.
 */
class hGPIO_int : public virtual hGPIO, public virtual IGPIO_int
{
	friend class hExt;
	friend class hSensor;

public:
	hGPIO_int(uint8_t nr);

	virtual void init() override;

	void interruptOff() override;

	void interruptSetEdge(InterruptEdge edge) override;

	bool interruptWait(uint32_t timeout = INFINITE) override;

protected:
	void _interruptOn(InterruptEdge edge, CountingMode mode, const AutoHandler& handler) override;
};

/**
 * @brief Implementation of on-board General Purpose Input-Output pin (GPIO) with ADC capability.
 */
class hGPIO_adc : public virtual hGPIO, public virtual IGPIO_adc
{
	friend class hExt;
	friend class hSensor;

public:
	hGPIO_adc(uint8_t nr);

	virtual void enableADC();
	virtual void disableADC();

	float analogReadVoltage();
	uint16_t analogReadRaw();
};

/**
 * @brief Implementation of on-board General Purpose Input-Output pin (GPIO) with interrupt and ADC capability.
 */
class hGPIO_super : public hGPIO_adc, public hGPIO_int, public virtual IGPIO_super
{
	friend class hExt;
	friend class hSensor;

public:
	hGPIO_super(uint8_t nr);

	void enableADC();
	void disableADC();
};

}
}

#endif /* __HGPIO_H__ */
