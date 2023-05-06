/*
 * This file is part of hFramework
 *
 * Copyright (C) 2014 Husarion Sp. z o.o. <husarion.com> -  All Rights Reserved
 *
 * Unauthorized copying of this file and the hFramework library, via any medium
 * is strictly prohibited.
 * Proprietary and confidential.
*/

#ifndef __HPRINTFDEV_H__
#define __HPRINTFDEV_H__

#undef putchar

#include <stdarg.h>

#include <hTypes.h>
#include <hStreamDev.h>
#include <hMutex.h>

namespace hFramework
{
/**
 * @brief Provides printf and vprintf methods as extension to hStreamDev
 */
class hPrintfDev : public virtual hStreamDev
{
public:
	/**
	 * @brief Write formatted string to the underlying device.
	 * @param str pointer to null terminated format string
	 * @param ... - optional arguments
	 * @return number of written characters
	 */
	int printf(const char* format, ...);

	/**
	 * @brief Write formatted string to the underlying device.
	 * @param str pointer to null terminated format string
	 * @param arg arguments as a va_list
	 * @return number of written characters
	 */
	virtual int vprintf(const char* format, va_list arg);

	/**
	 * @brief Put single character.
	 * @param ch character
	 */
	int putchar(char c);

	/**
	 * @brief Return single character.
	 * @return character.
	 */
	int getch();

protected:
	hMutex printfMutex;
};

}
#endif //__HPRINTFDEV_H__
