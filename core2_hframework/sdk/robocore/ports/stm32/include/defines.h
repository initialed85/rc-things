/*
 * This file is part of hFramework
 *
 * Copyright (C) 2014 Husarion Sp. z o.o. <husarion.com> -  All Rights Reserved
 *
 * Unauthorized copying of this file and the hFramework library, via any medium
 * is strictly prohibited.
 * Proprietary and confidential.
*/

#ifndef __DEFINES_H__
#define __DEFINES_H__

#define ROBOCORE  2
#define CORE2     3
#define CORE2MINI 4

#define BOARD(type) (type == BOARD_TYPE)

//// ROBOCORE
#if BOARD(ROBOCORE)
#define CAN_ENABLED
#define HAS_HEXT 1

//// CORE2
#elif BOARD(CORE2)
#define CAN_ENABLED
#define SERVOMODULE_ENABLED
#define HAS_HEXT 1
#define SERVOS_COUNT 6

//// CORE2 mini
#elif BOARD(CORE2MINI)
#define SERVOMODULE_ENABLED
#define SERVOS_COUNT 4

#else
#  error no board
#endif

#endif
