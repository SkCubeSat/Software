/** @file cubeObc_bootloader.h
 *
 * @brief libCubeObc CubeSpace bootloader common operation helpers header file
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

#ifndef CUBEOBC_BOOTLOADER__H
#define CUBEOBC_BOOTLOADER__H

#ifdef __cplusplus
extern "C" {
#endif

/***************************** SYSTEM INCLUDES *******************************/

#include <cubeObc/cubeObc.h>

/************************ DEPENDENT MODULE INCLUDES **************************/

#include "tctlmCommonFramework1.h"
#include "tctlmCubeCommonBaseBootloader5.h"

/***************************** GLOBAL DEFINES ********************************/

/**************************** GLOBAL CONSTANTS *******************************/

/****************************** GLOBAL MACROS ********************************/

/***************************** GLOBAL TYPEDEFS *******************************/

/***************************** GLOBAL VARIABLES ******************************/

/***************************** GLOBAL FUNCTIONS ******************************/

/**
 * @brief Poll bootloader State telemetry for a specified state
 *
 * @note This will return immediately if a single State request fails
 *
 * @param[in]	endpoint	Comms endpoint
 * @param[in]	state		State to poll for
 * @param[in]	backoff		Backoff time between polls [ms]
 * @param[in]	timeout		Total time to wait for state [ms]
 * @param[out]	error		Flag indicating if state result suggests an error occurred
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_bootloader_pollForState(TypeDef_TctlmEndpoint *endpoint,
										  TctlmCubeCommonBaseBootloader5_States state,
										  U32 backoff, U32 timeout, Boolean *error);

/**
 * @brief Upload a CubeSpace (.cs) file to bootloader flash
 *
 * @param[in]	endpoint	Comms endpoint
 * @param[in]	size		The size of the CubeSpace file
 * @param[in]	userData	User specific data
 * @param[out]	errors		The last requested Errors telemetry for debugging if return is not CUBEOBC_ERROR_OK.
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_bootloader_uploadCubeSpaceFile(TypeDef_TctlmEndpoint *endpoint, U32 size, void *userData,
												 TctlmCubeCommonBaseBootloader5_Errors *errors);

/*****************************************************************************/

#ifdef __cplusplus
}
#endif
#endif /* CUBEOBC_BOOTLOADER__H */
