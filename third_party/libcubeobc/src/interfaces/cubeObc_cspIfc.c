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

#include <cubeObc/interfaces/cubeObc_cspIfc.h>

/***************************** MODULE DEFINES ********************************/

/****************************** MODULE MACROS ********************************/

/***************************** MODULE TYPEDEFS *******************************/

/***************************** MODULE VARIABLES ******************************/

/***************************** MODULE FUNCTIONS ******************************/

/***************************** GLOBAL FUNCTIONS ******************************/

WEAK ErrorCode cubeObc_cspIfc_recvFrom(U8 port, U8 *data, U16 *dataSize, U32 timeout)
{
	(void)port;
	(void)data;
	(void)dataSize;
	(void)timeout;

	// See cubeObc_example_CSP.c for an example of how to implement this callback

	return CUBEOBC_ERROR_TODO;
}

WEAK ErrorCode cubeObc_cspIfc_sendTo(U8 dst, U8 dstPort, U8 srcPort, U8 *data, U16 dataSize, U32 timeout)
{
	(void)dst;
	(void)dstPort;
	(void)srcPort;
	(void)data;
	(void)dataSize;
	(void)timeout;

	// See cubeObc_example_CSP.c for an example of how to implement this callback

	return CUBEOBC_ERROR_TODO;
}

/*** end of file ***/
