/*
 * This file is part of hFramework
 *
 * Copyright (C) 2014 Husarion Sp. z o.o. <husarion.com> -  All Rights Reserved
 *
 * Unauthorized copying of this file and the hFramework library, via any medium
 * is strictly prohibited.
 * Proprietary and confidential.
*/

#ifndef __HPACKETDEV_H__
#define __HPACKETDEV_H__

class hPacketDev
{
public:
	enum class Result { Success = 0, Timeout = -1, Error = -2 };

	virtual ~hPacketDev() { }

	virtual int readPacket(void* data, int maxLength, int timeout) = 0;

	virtual bool beginPacket() = 0;
	virtual bool write(const void* data, int length) = 0;
	virtual bool endPacket() = 0;
};

#endif
