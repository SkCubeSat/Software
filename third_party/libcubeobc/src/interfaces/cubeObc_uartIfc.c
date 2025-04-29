/** @file cubeObc_uartIfc.c
 *
 * @brief UART interface (libCubeObc <-> OBC hardware)
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

/***************************** SYSTEM INCLUDES *******************************/

#include <cubeObc/cubeObc_errorDef.h>

/***************************** MODULE INCLUDES *******************************/

#include <cubeObc/interfaces/cubeObc_uartIfc.h>

/***************************** MODULE DEFINES ********************************/

/****************************** MODULE MACROS ********************************/

/***************************** MODULE TYPEDEFS *******************************/

/***************************** MODULE VARIABLES ******************************/

/***************************** MODULE FUNCTIONS ******************************/

/***************************** GLOBAL FUNCTIONS ******************************/

WEAK void cubeObc_uartIfc_rxFlush(void) {}

WEAK ErrorCode cubeObc_uartIfc_rx(U8 *data, U32 size, U32 *sizeRead)
{
	(void)data;
	(void)size;
	(void)sizeRead;

	return CUBEOBC_ERROR_TODO;
}

WEAK ErrorCode cubeObc_uartIfc_tx(CONST U8 *data, U32 size)
{
	(void)data;
	(void)size;

	return CUBEOBC_ERROR_TODO;
}

/*** end of file ***/
