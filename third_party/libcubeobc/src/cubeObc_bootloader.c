/** @file cubeObc_bootloader.c
 *
 * @brief libCubeObc CubeSpace bootloader common operation helpers
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

/***************************** SYSTEM INCLUDES *******************************/

/***************************** MODULE INCLUDES *******************************/

#include <cubeObc/cubeObc_bootloader.h>
#include <cubeObc/cubeObc_bulkDataTransfer.h>

/***************************** MODULE DEFINES ********************************/

/****************************** MODULE MACROS ********************************/

/***************************** MODULE TYPEDEFS *******************************/

/***************************** MODULE VARIABLES ******************************/

/***************************** MODULE FUNCTIONS ******************************/

/***************************** GLOBAL FUNCTIONS ******************************/

ErrorCode cubeObc_bootloader_pollForState(TypeDef_TctlmEndpoint *endpoint,
										  TctlmCubeCommonBaseBootloader5_States state,
										  U32 backoff, U32 timeout, Boolean *error)
{
	ErrorCode result;
	Boolean done = FALSE;
	U16 backoffTotal = 0u; // How long we have waited in total for initialization to complete

	do
	{
		TctlmCubeCommonBaseBootloader5_State appState;

		result = tctlmCubeCommonBaseBootloader5_getState(endpoint, &appState);

		if (result == CUBEOBC_ERROR_OK)
		{
			*error = (appState.result != CUBEOBC_ERROR_OK);

			done = ((appState.appState == state) || (appState.result != CUBEOBC_ERROR_OK));

			if (done == FALSE)
			{
				if (backoffTotal >= timeout)
				{
					result = CUBEOBC_ERROR_TOUT;
				}
				else
				{
					cubeObc_time_delay(backoff);

					backoffTotal += backoff; // Increment how many milliseconds we have waited
				}
			}
		}
	}
	while ((result == CUBEOBC_ERROR_OK) && (done == FALSE));

	return result;
}

ErrorCode cubeObc_bootloader_uploadCubeSpaceFile(TypeDef_TctlmEndpoint *endpoint, U32 size, void *userData,
												 TctlmCubeCommonBaseBootloader5_Errors *errors)
{
	if ((endpoint == NULL) || (errors == NULL))
		return CUBEOBC_ERROR_NULLPTR;

	if (endpoint->nodeType == TCTLM_COMMON_FRAMEWORK_ENUMS__NODE_TYPE_INVALID)
		return CUBEOBC_ERROR_NODE_TYPE;

	ErrorCode result;
	U16 metaSize;
	U32 dataSize;
	U8 *data;

	ZERO_VAR(*errors);

	// Request a new frame buffer from OBC - only 2 bytes to get the size of the meta data
	result = cubeObc_bulkDataTransfer_getFrameBuffer(userData, &data, sizeof(U16));

	if (result == CUBEOBC_ERROR_OK)
	{
		metaSize = *((U16 *)data); // The size of the meta data is in the first 2 bytes of the file
		dataSize = size - metaSize;

		// Request a new frame buffer from OBC - for the meta data
		// Note that we do not commit the previous frame so that we get this frame buffer also from the start of the file
		result = cubeObc_bulkDataTransfer_getFrameBuffer(userData, &data, metaSize);
	}

	if (result == CUBEOBC_ERROR_OK)
	{
		// Tell the OBC we have used "metaSize" frame and buffer index should be updated
		result = cubeObc_bulkDataTransfer_commitFrameBuffer(userData, data, metaSize);
	}

	if (result == CUBEOBC_ERROR_OK)
	{
		result = tctlmCubeCommonBaseBootloader5_setWriteFileSetup(endpoint, data);
	}

	if (result == CUBEOBC_ERROR_OK)
	{
		Boolean error;

		// Upload to internal flash requires 30000ms to initialize to cater for all cases
		result = cubeObc_bootloader_pollForState(endpoint, TCTLM_CUBE_COMMON_BASE_BOOTLOADER_5__STATE_BUSY_WAIT_FRAME,
												 200u, 30000u, &error);

		if ((result == CUBEOBC_ERROR_OK) &&
			(error == TRUE))
		{
			// Requests for <State> were successful, but there was an internal error
			// The caller should inspect <Errors> telemetry for details of the error
			(void)tctlmCubeCommonBaseBootloader5_getErrors(endpoint, errors);

			return CUBEOBC_ERROR_FTP;
		}
	}

	if (result == CUBEOBC_ERROR_OK)
	{
		// Only pass the data portion of the file to be uploaded
		result = cubeObc_bulkDataTransfer_upload(endpoint, userData, dataSize);

		// Request errors one final time after the upload to capture any errors that may have occurred
		(void)tctlmCubeCommonBaseBootloader5_getErrors(endpoint, errors);
	}

	return result;
}

// TODO Raw memory access

/*** end of file ***/
