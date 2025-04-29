/** @file cubeObc_bulkDataTransfer.c
 *
 * @brief libCubeObc Bulk Data Transfer Handler
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

/***************************** SYSTEM INCLUDES *******************************/

/***************************** MODULE INCLUDES *******************************/

#include <cubeObc/cubeObc_bulkDataTransfer.h>

#include "tctlmCommonTransfer1.h"

/***************************** MODULE DEFINES ********************************/

/**
 * @brief Internal timeout between TransferFrame telecommands before the CubeProduct will cancel the transfer
 */
#define BDT_TIMEOUT			((U32)1000u)

/****************************** MODULE MACROS ********************************/

/***************************** MODULE TYPEDEFS *******************************/

/***************************** MODULE VARIABLES ******************************/

/***************************** MODULE FUNCTIONS ******************************/

/**
 * @brief CRC calculation for frame data
 *
 * @param[in]	data	Frame data
 * @param[in]	size	Drame data size
 *
 * @return CRC over frame data
 */
PRIVATE U8 crc(U8 *data, U16 size)
{
	U8 checksum = 0xFFu;

	for (U16 i = 0; i < size; i++)
	{
		checksum ^= data[i];
	}

	return checksum;
}

/**
 * @brief Sets the current frame number via TransferFrame telecommand
 *
 * @param[in]		endpoint			Comms endpoint
 * @param[in,out]	lastFrameSentTime	Timestamp of the last time the TransferFrame telecommand was successfully sent - used for timeout
 * @param[in]		frameNumber			Frame number to set
 *
 * @return The last result of tctlmCommonTransfer1_setTransferFrame().
 *
 * @return CUBEOBC_ERROR_OK - if no error.
 */
PRIVATE ErrorCode setFrameNumber(TypeDef_TctlmEndpoint *endpoint, U32 *lastFrameSentTime, U16 frameNumber)
{
	ErrorCode result;
	ErrorCode lastResult = CUBEOBC_ERROR_OK;
	U32 transferFrameSentTime = *lastFrameSentTime;
	Boolean shouldExit = FALSE;
	Boolean frameSet = FALSE;

	do
	{
		result = tctlmCommonTransfer1_setTransferFrame(endpoint, frameNumber);

		if (result == CUBEOBC_ERROR_OK)
		{
			// Capture the time when the last TransferFrame was successful
			transferFrameSentTime = cubeObc_time_getMs();

			frameSet = TRUE;
		}
		else
		{
			U32 timeNow = cubeObc_time_getMs();

			if (result == CUBEOBC_ERROR_TOUT)
			{
				if ((timeNow - transferFrameSentTime) > BDT_TIMEOUT)
				{
					shouldExit = TRUE;
				}
				else
				{
					// Do nothing, retry immediately
				}
			}
			else if (result == CUBEOBC_ERROR_TCTLM_BUSY)
			{
				if (lastResult == CUBEOBC_ERROR_TOUT)
				{
					// If the last exception was a timeout, and then we receive a busy nack
					// it suggests that the frame number was set correctly but did not receive a response, but
					// the frame is still being processed
					// so we assume that's the case, and continue as usual to get FrameInfo
					frameSet = TRUE;
				}
				else if ((timeNow - transferFrameSentTime) > BDT_TIMEOUT)
				{
					shouldExit = TRUE;
				}
				else
				{
					// Backoff 5ms and retry
					cubeObc_time_delay(5u);
				}
			}
			else if (result == CUBEOBC_ERROR_TCTLM_INVALID_PARAM)
			{
				if (lastResult == CUBEOBC_ERROR_TOUT)
				{
					// If the last exception was a timeout, and then we receive a busy nack
					// it suggests that the frame number was set correctly but did not receive a response, but
					// the frame is still being processed
					// so we assume that's the case, and continue as usual to get FrameInfo
					frameSet = TRUE;
				}
				else if ((timeNow - transferFrameSentTime) > BDT_TIMEOUT)
				{
					shouldExit = TRUE;
				}
				else
				{
					// Do nothing, retry immediately
				}
			}
			else
			{
				// Unexpected error, exit
				shouldExit = TRUE;
			}
		}

		lastResult = result;
	}
	while ((frameSet == FALSE) && (shouldExit == FALSE));

	// Pass the last TransferFrame timestamp to caller
	*lastFrameSentTime = transferFrameSentTime;

	if (frameSet == TRUE)
	{
		result = CUBEOBC_ERROR_OK;
	}

	return result;
}

/**
 * @brief Polls the FrameInfo telemetry for an update of the frame number
 *
 * @param[in]		endpoint			Comms endpoint
 * @param[in,out]	lastFrameSentTime	Timestamp of the last time the TransferFrame telecommand was successfully sent - used for timeout
 * @param[out]		frameLast			Indicates if this is the last frame in the transfer
 * @param[out]		frameError			Indicates if an error occurred processing the current frame
 * @param[in]		frameNumber			Frame number to set
 *
 * @return The last result of tctlmCommonTransfer1_getFrameInfo() if it was unsuccessful.
 *
 * @return CUBEOBC_ERROR_TOUT - if the time since "lastFrameSentTime", exceeds BDT_TIMEOUT.
 *
 * @return CUBEOBC_ERROR_OK - if no error.
 */
PRIVATE ErrorCode pollFrameNumber(TypeDef_TctlmEndpoint *endpoint, U32 *lastFrameSentTime,
								  Boolean *frameLast, Boolean *frameError, U16 frameNumber)
{
	ErrorCode result;
	U32 transferFrameSentTime = *lastFrameSentTime;
	Boolean frameProcessed = FALSE;
	Boolean frameLastLoc = FALSE;
	Boolean frameErrorLoc = FALSE;
	Boolean shouldExit = FALSE;

	do
	{
		// Back-off 10ms
		cubeObc_time_delay(10u);

		TctlmCommonTransfer1_FrameInfo frameInfo;

		result = tctlmCommonTransfer1_getFrameInfo(endpoint, &frameInfo);

		if (result == CUBEOBC_ERROR_OK)
		{
			frameProcessed = (frameInfo.frameNumber == frameNumber);
			frameLastLoc = frameInfo.frameLast;
			frameErrorLoc = frameInfo.frameError;
		}

		if (frameProcessed == FALSE)
		{
			U32 timeNow = cubeObc_time_getMs();

			if ((timeNow - transferFrameSentTime) > BDT_TIMEOUT)
			{
				shouldExit = TRUE;

				if (result == CUBEOBC_ERROR_OK)
				{
					result = CUBEOBC_ERROR_TOUT;
				}
			}
		}
	}
	while ((frameProcessed == FALSE) && (frameErrorLoc == FALSE) && (shouldExit == FALSE));

	*frameLast = frameLastLoc;
	*frameError = frameErrorLoc;

	return result;
}

/***************************** GLOBAL FUNCTIONS ******************************/

ErrorCode cubeObc_bulkDataTransfer_download(TypeDef_TctlmEndpoint *endpoint, void *userData)
{
	ErrorCode result;
	U32 transferFrameSentTime = cubeObc_time_getMs();
	Boolean frameLast = FALSE;
	Boolean frameError = FALSE;
	U16 frameNumber = 0u;

	do
	{
		result = setFrameNumber(endpoint, &transferFrameSentTime, frameNumber);

		if (result == CUBEOBC_ERROR_OK)
		{
			result = pollFrameNumber(endpoint, &transferFrameSentTime, &frameLast, &frameError, frameNumber);
		}

		if ((result == CUBEOBC_ERROR_OK) &&
			(frameError == TRUE))
		{
			result = CUBEOBC_ERROR_FRAME;
		}

		if (result == CUBEOBC_ERROR_OK)
		{
			Boolean shouldExit = FALSE;
			TctlmCommonTransfer1_Frame frame;

			do
			{
				result = tctlmCommonTransfer1_getFrame(endpoint, &frame);

				if (result == CUBEOBC_ERROR_OK)
				{
					shouldExit = TRUE;
				}
				else
				{
					U32 timeNow = cubeObc_time_getMs();

					if ((timeNow - transferFrameSentTime) > BDT_TIMEOUT)
					{
						shouldExit = TRUE;
					}
				}
			}
			while (shouldExit == FALSE);

			if (result == CUBEOBC_ERROR_OK)
			{
				if ((frame.frameSize == 0u) && (frameLast == FALSE))
				{
					// This is never an expected condition
					result = CUBEOBC_ERROR_UNKNOWN;
				}
			}

			if ((result == CUBEOBC_ERROR_OK) && (frame.frameSize > 0u))
			{
				U8 *data;

				// Request a new frame buffer from OBC
				result = cubeObc_bulkDataTransfer_getFrameBuffer(userData, &data, frame.frameSize);

				if (result == CUBEOBC_ERROR_OK)
				{
					// Copy data to buffer
					MEMCPY(data, frame.frameBytes, frame.frameSize);

					// Tell the OBC the frame is populated and can be used
					result = cubeObc_bulkDataTransfer_commitFrameBuffer(userData, data, frame.frameSize);

					frameNumber++;
				}
			}
		}
	}
	while ((result == CUBEOBC_ERROR_OK) && (frameLast == FALSE));

	return result;
}

ErrorCode cubeObc_bulkDataTransfer_upload(TypeDef_TctlmEndpoint *endpoint, void *userData, U32 size)
{
	ErrorCode result = CUBEOBC_ERROR_OK;
	U32 transferFrameSentTime = cubeObc_time_getMs();
	Boolean frameLast = FALSE;
	Boolean frameError = FALSE;
	U16 frameNumber = 0u;
	U32 dataRemain = size;

	while ((result == CUBEOBC_ERROR_OK) &&
		   (dataRemain > 0u))
	{
		Boolean frameSent = FALSE;
		Boolean shouldExit = FALSE;
		TctlmCommonTransfer1_Frame frame;
		U8 *data;

		frame.frameSize = (dataRemain > BDT_MAX_FRAME_SIZE) ? BDT_MAX_FRAME_SIZE : (U16)dataRemain;

		// Request a new frame buffer from OBC
		result = cubeObc_bulkDataTransfer_getFrameBuffer(userData, &data, frame.frameSize);

		if (result == CUBEOBC_ERROR_OK)
		{
			MEMCPY(frame.frameBytes, data, frame.frameSize);

			do
			{
				result = tctlmCommonTransfer1_setFrame(endpoint, &frame);

				if (result == CUBEOBC_ERROR_OK)
				{
					TctlmCommonTransfer1_FrameInfo frameInfo;

					result = tctlmCommonTransfer1_getFrameInfo(endpoint, &frameInfo);

					if (result == CUBEOBC_ERROR_OK)
					{
						U8 checksum = crc(frame.frameBytes, frame.frameSize);

						if (frameInfo.checkSum == checksum)
						{
							frameSent = TRUE;

							// Tell the OBC the frame has been sent
							result = cubeObc_bulkDataTransfer_commitFrameBuffer(userData, data, frame.frameSize);
						}
						else
						{
							result = CUBEOBC_ERROR_CRC;

							shouldExit = TRUE;
						}
					}
				}

				if ((result != CUBEOBC_ERROR_OK) && (shouldExit == FALSE))
				{
					U32 timeNow = cubeObc_time_getMs();

					if ((timeNow - transferFrameSentTime) > BDT_TIMEOUT)
					{
						shouldExit = TRUE;
					}
				}
			}
			while ((frameSent == FALSE) && (shouldExit == FALSE));
		}

		if (result == CUBEOBC_ERROR_OK)
		{
			result = setFrameNumber(endpoint, &transferFrameSentTime, frameNumber);
		}

		if (result == CUBEOBC_ERROR_OK)
		{
			result = pollFrameNumber(endpoint, &transferFrameSentTime, &frameLast, &frameError, frameNumber);
		}

		if (result == CUBEOBC_ERROR_OK)
		{
			if (frameError == TRUE)
			{
				result = CUBEOBC_ERROR_FRAME;
			}
		}

		if (result == CUBEOBC_ERROR_OK)
		{
			dataRemain -= frame.frameSize;
			frameNumber++;
		}
	}

	return result;
}

WEAK ErrorCode cubeObc_bulkDataTransfer_getFrameBuffer(void *userData, U8 **frame, U16 size)
{
	(void)userData;
	(void)frame;
	(void)size;

	/*
	 * EXAMPLE CODE - ASSUMING OBC IS RESOURCE CONSTRANED AND MUST READ/WRITE STORAGE DURING OPERATION
	 *
	 * U8 *filePtr = (U8 *)userData;
	 * U8 frameBuffer[BDT_MAX_FRAME_SIZE];
	 *
	 * CHECK(size <= BDT_MAX_FRAME_SIZE);
	 *     return CUBEOBC_ERROR_SIZE; // if not
	 *
	 * CHECK((filePtr + size) < fileSize); // where fileSize must be determined by OBC before start
	 *     return CUBEOBC_ERROR_SIZE; // if not
	 *
	 * READ_FROM_FILE(filePtr, frameBuffer, size);
	 *
	 * *frame = frameBuffer;
	 */

	return CUBEOBC_ERROR_TODO;
}

WEAK ErrorCode cubeObc_bulkDataTransfer_commitFrameBuffer(void *userData, U8 *frame, U16 size)
{
	(void)userData;
	(void)frame;
	(void)size;

	/*
	 * EXAMPLE CODE - ASSUMING OBC IS RESOURCE CONSTRANED AND MUST READ/WRITE STORAGE DURING OPERATION
	 *
	 * U8 *filePtr = (U8 *)userData;
	 * U8 frameBuffer[BDT_MAX_FRAME_SIZE];
	 *
	 * CHECK(frame == frameBuffer); // An example of validation of the frame being committed (depends on OBC implementation)
	 *    return CUBEOBC_ERROR_COMMIT; // if not
	 *
	 * CHECK(size == <size of provided buffer>); // An example of validation of the frame being committed
	 *                                                   // (depends on OBC implementation)
	 *     return CUBEOBC_ERROR_COMMIT; // if not
	 *
	 * filePtr += size; // Move file pointer now that frame is processed
	 */

	return CUBEOBC_ERROR_TODO;
}

/*** end of file ***/
