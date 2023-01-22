/*
 * This file is part of hFramework
 *
 * Copyright (C) 2014 Husarion Sp. z o.o. <husarion.com> -  All Rights Reserved
 *
 * Unauthorized copying of this file and the hFramework library, via any medium
 * is strictly prohibited.
 * Proprietary and confidential.
*/

#ifndef __HCONDVAR_H__
#define __HCONDVAR_H__

#include <hTypes.h>

#include <hMutex.h>

namespace hFramework
{

class hCondVar
{

public:
	hCondVar();
	~hCondVar();

	void notifyOne();
	bool wait(hMutex& mutex, uint32_t timeout = INFINITE);

private:
	class hCondVarPimpl;
	hCondVarPimpl* impl;
	
	hCondVar(const hCondVar&) = delete;

};
}


#endif
