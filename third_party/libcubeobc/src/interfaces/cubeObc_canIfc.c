/** @file cubeObc_canIfc.c
 *
 * @brief CAN bus interface (libCubeObc <-> OBC hardware)
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

/***************************** SYSTEM INCLUDES *******************************/

#include <cubeObc/cubeObc_errorDef.h>

/***************************** MODULE INCLUDES *******************************/

#include <cubeObc/interfaces/cubeObc_canIfc.h>

/***************************** MODULE DEFINES ********************************/

/****************************** MODULE MACROS ********************************/

/***************************** MODULE TYPEDEFS *******************************/

/***************************** MODULE VARIABLES ******************************/

/***************************** MODULE FUNCTIONS ******************************/

/***************************** GLOBAL FUNCTIONS ******************************/

WEAK void cubeObc_canIfc_rxFlush(void) {}

WEAK ErrorCode cubeObc_canIfc_rx(CanPacket *packet)
{
	(void)packet;

	return CUBEOBC_ERROR_TODO;
}

WEAK ErrorCode cubeObc_canIfc_tx(CONST CanPacket *packet)
{
	(void)packet;

	return CUBEOBC_ERROR_TODO;
}

/*** end of file ***/
