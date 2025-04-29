/** @file cubeObc_can.h
 *
 * @brief CAN driver for libcubeobc
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

#ifndef CUBEOBC_CAN__H
#define CUBEOBC_CAN__H

#ifdef __cplusplus
extern "C" {
#endif

/***************************** SYSTEM INCLUDES *******************************/

#include <cubeObc/cubeObc_typeDef.h>

/************************ DEPENDENT MODULE INCLUDES **************************/

/***************************** GLOBAL DEFINES ********************************/

/**************************** GLOBAL CONSTANTS *******************************/

/****************************** GLOBAL MACROS ********************************/

/***************************** GLOBAL TYPEDEFS *******************************/

/***************************** GLOBAL VARIABLES ******************************/

/***************************** GLOBAL FUNCTIONS ******************************/

/**
 * @brief CAN driver initializer
 *
 * @param[in]	device CAN device
 *
 * @return CUUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_can_init(Text *device);

/**
 * @brief Receive CAN packet
 *
 * @param[out]	packet	Received CAN packet
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_can_rx(CanPacket *packet);

/**
 * @brief Transmit CAN packet
 *
 * @param[in]	packet	CAN packet to transmit
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_can_tx(CONST CanPacket *packet);

#ifdef __cplusplus
}
#endif
#endif /* CUBEOBC_CAN__H */
