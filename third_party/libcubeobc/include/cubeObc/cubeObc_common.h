/** @file cubeObc_common.h
 *
 * @brief libCubeObc CubeProduct common operation helpers header file
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

#ifndef CUBEOBC_COMMON__H
#define CUBEOBC_COMMON__H

#ifdef __cplusplus
extern "C" {
#endif

/***************************** SYSTEM INCLUDES *******************************/

#include <cubeObc/cubeObc.h>

/************************ DEPENDENT MODULE INCLUDES **************************/

#include "tctlmCommonFramework1.h"

/***************************** GLOBAL DEFINES ********************************/

/**************************** GLOBAL CONSTANTS *******************************/

/****************************** GLOBAL MACROS ********************************/

/***************************** GLOBAL TYPEDEFS *******************************/

/***************************** GLOBAL VARIABLES ******************************/

/***************************** GLOBAL FUNCTIONS ******************************/

/**
 * @brief Poll BootStatus telemetry for specified state or not specified state
 *
 * @param[in]	endpoint	Comms endpoint
 * @param[in]	state		State to poll for
 * @param[in]	backoff		Backoff between requests for BootStatus
 * @param[in]	timeout		Timeout to wait for state
 * @param[in]	notState	Set TRUE if polling for BootState != state, otherwise FALSE
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_common_pollForBootState(TypeDef_TctlmEndpoint *endpoint,
										  TctlmCommonFramework1_BootState state,
										  U32 backoff, U32 timeout,
										  Boolean notState);

/*****************************************************************************/

#ifdef __cplusplus
}
#endif
#endif /* CUBEOBC_COMMON__H */
