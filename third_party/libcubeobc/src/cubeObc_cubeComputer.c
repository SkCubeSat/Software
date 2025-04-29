/** @file cubeObc_bootloader.c
 *
 * @brief libCubeObc CubeComputer operation helpers
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

/***************************** SYSTEM INCLUDES *******************************/

/***************************** MODULE INCLUDES *******************************/

#include <cubeObc/cubeObc_cubeComputer.h>
#include <cubeObc/cubeObc_bulkDataTransfer.h>

/***************************** MODULE DEFINES ********************************/

/****************************** MODULE MACROS ********************************/

/***************************** MODULE TYPEDEFS *******************************/

/***************************** MODULE VARIABLES ******************************/

/***************************** MODULE FUNCTIONS ******************************/

/************************** GLOBAL COMMON FUNCTIONS **************************/

ErrorCode cubeObc_cubeComputer_pollForFtpState(TypeDef_TctlmEndpoint *endpoint,
											   TctlmCubeComputerControlProgram8_FtpState state,
											   U32 backoff, U32 timeout,
											   TctlmCubeComputerControlProgram8_FileTransferStatus *status)
{
	ErrorCode result;
	Boolean done = FALSE;
	U16 backoffTotal = 0u; // How long we have waited in total for initialization to complete

	do
	{
		result = tctlmCubeComputerControlProgram8_getFileTransferStatus(endpoint, status);

		if (result == CUBEOBC_ERROR_OK)
		{
			done = ((status->state == state) || (status->errorCode != CUBEOBC_ERROR_OK));

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

ErrorCode cubeObc_cubeComputer_pollForFtpUpgradeState(TypeDef_TctlmEndpoint *endpoint,
													  TctlmCubeComputerControlProgram8_FtpUpgradeState state,
													  U32 backoff, U32 timeout,
													  TctlmCubeComputerControlProgram8_FileTransferStatus *status)
{
	ErrorCode result;
	Boolean done = FALSE;
	U16 backoffTotal = 0u; // How long we have waited in total for initialization to complete

	do
	{
		result = tctlmCubeComputerControlProgram8_getFileTransferStatus(endpoint, status);

		if (result == CUBEOBC_ERROR_OK)
		{
			done = ((status->upgradeState == state) || (status->errorCode != CUBEOBC_ERROR_OK));

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

ErrorCode cubeObc_cubeComputer_uploadCubeSpaceFile(TypeDef_TctlmEndpoint *endpoint, U32 size, void *userData,
												   TctlmCubeComputerControlProgram8_FileTransferStatus *status)
{
	if ((endpoint == NULL) || (status == NULL))
		return CUBEOBC_ERROR_NULLPTR;

	if (endpoint->nodeType == TCTLM_COMMON_FRAMEWORK_ENUMS__NODE_TYPE_INVALID)
		return CUBEOBC_ERROR_NODE_TYPE;

	ErrorCode result;
	U16 metaSize;
	U32 dataSize;
	TctlmCubeComputerControlProgram8_FileTransferSetup setup;
	U8 *data;

	ZERO_VAR(*status);

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
		ZERO_VAR(setup); // Zero all unused parameters

		setup.opCode = TCTLM_CUBE_COMPUTER_CONTROL_PROGRAM_8__FTP_UPLOAD;
		MEMCPY(setup.metaData, data, metaSize); // Copy meta data to setup

		result = tctlmCubeComputerControlProgram8_setFileTransferSetup(endpoint, &setup);
	}

	if (result == CUBEOBC_ERROR_OK)
	{
		// CubeComputer upload to external storage requires 1000ms to initialize
		result = cubeObc_cubeComputer_pollForFtpState(endpoint, TCTLM_CUBE_COMPUTER_CONTROL_PROGRAM_8__BUSY,
													  50u, 1000u, status);

		if ((result == CUBEOBC_ERROR_OK) &&
			(status->errorCode != CUBEOBC_ERROR_OK))
		{
			// Requests for <FileTransferStatus> were successful, but there was an internal error
			// The caller should inspect the returned "status" for details of the error
			return CUBEOBC_ERROR_FTP;
		}
	}

	if (result == CUBEOBC_ERROR_OK)
	{
		// Only pass the data portion of the file to be uploaded
		result = cubeObc_bulkDataTransfer_upload(endpoint, userData, dataSize);

		// Request status one final time after the upload to capture any errors that may have occurred
		(void)tctlmCubeComputerControlProgram8_getFileTransferStatus(endpoint, status);
	}

	return result;
}

ErrorCode cubeObc_cubeComputer_upgrade(TypeDef_TctlmEndpoint *endpoint,
									   TctlmCommonFrameworkEnums_NodeType nodeType, U32 serialInt,
									   TctlmCommonFrameworkEnums_ProgramType program,
									   TctlmCubeComputerControlProgram8_NodePort forcePort,
									   TctlmCubeComputerControlProgram8_FileTransferStatus *status)
{
	if ((endpoint == NULL) || (status == NULL))
		return CUBEOBC_ERROR_NULLPTR;

	if (endpoint->nodeType == TCTLM_COMMON_FRAMEWORK_ENUMS__NODE_TYPE_INVALID)
		return CUBEOBC_ERROR_NODE_TYPE;

	// This routine is only for upgrading CubeComputer or nodes via CubeComputer
	// To upgrade stand-alone CubeProducts that are not CubeComputer see cubeObc_bootloader.c
	if (endpoint->nodeType != TCTLM_COMMON_FRAMEWORK_ENUMS__NODE_TYPE_CUBE_COMPUTER)
		return CUBEOBC_ERROR_NODE_TYPE;

	if (forcePort != TCTLM_CUBE_COMPUTER_CONTROL_PROGRAM_8__PORT_NONE)
	{
		// Force port upgrades do not apply to upgrading CubeComputer itself
		if (nodeType == TCTLM_COMMON_FRAMEWORK_ENUMS__NODE_TYPE_CUBE_COMPUTER)
			return CUBEOBC_ERROR_USAGE;

		// Force port upgrades are only applicable to node bootloader upgrades
		// Once the node bootloader is programmed, it should be found by auto-discovery, the force port is not required.
		if (program != TCTLM_COMMON_FRAMEWORK_ENUMS__PROGRAM_TYPE_BOOTLOADER)
			return CUBEOBC_ERROR_USAGE;
	}

	ErrorCode result;
	TctlmCubeComputerControlProgram8_FileTransferSetup setup;

	ZERO_VAR(*status);
	ZERO_VAR(setup); // Zero all unused parameters

	setup.opCode = TCTLM_CUBE_COMPUTER_CONTROL_PROGRAM_8__FTP_UPGRADE;
	setup.node = nodeType;
	setup.serialInt = serialInt;
	setup.program = program;
	setup.forcePort = forcePort;

	result = tctlmCubeComputerControlProgram8_setFileTransferSetup(endpoint, &setup);

	if (result == CUBEOBC_ERROR_OK)
	{
		result = cubeObc_cubeComputer_pollForFtpUpgradeState(endpoint, TCTLM_CUBE_COMPUTER_CONTROL_PROGRAM_8__UPGRADE_IDLE,
															 500u, 120000u, status);

		if ((result == CUBEOBC_ERROR_OK) &&
			(status->errorCode != CUBEOBC_ERROR_OK))
		{
			// Requests for <FileTransferStatus> were successful, but there was an internal error
			// The caller should inspect the returned "status" for details of the error
			return CUBEOBC_ERROR_FTP;
		}
	}

	return result;
}

ErrorCode cubeObc_cubeComputer_getFileInfo(TypeDef_TctlmEndpoint *endpoint,
										   TctlmCubeComputerControlProgram8_FtpFiles file,
										   TctlmCubeComputerControlProgram8_FileInfo *info,
										   TctlmCubeComputerControlProgram8_FileTransferStatus *status)
{
	if ((endpoint == NULL) || (info == NULL) || (status == NULL))
		return CUBEOBC_ERROR_NULLPTR;

	ErrorCode result;
	TctlmCubeComputerControlProgram8_FileTransferSetup setup;

	ZERO_VAR(*status);
	ZERO_VAR(*info);
	ZERO_VAR(setup); // Zero all unused parameters

	setup.opCode = TCTLM_CUBE_COMPUTER_CONTROL_PROGRAM_8__FTP_INFO;
	setup.file = file;

	result = tctlmCubeComputerControlProgram8_setFileTransferSetup(endpoint, &setup);

	if (result == CUBEOBC_ERROR_OK)
	{
		result = cubeObc_cubeComputer_pollForFtpState(endpoint, TCTLM_CUBE_COMPUTER_CONTROL_PROGRAM_8__IDLE,
													  50u, 3000u, status);

		if ((result == CUBEOBC_ERROR_OK) &&
			(status->errorCode != CUBEOBC_ERROR_OK))
		{
			// Requests for <FileTransferStatus> were successful, but there was an internal error
			// The caller should inspect the returned "status" for details of the error
			return CUBEOBC_ERROR_FTP;
		}
	}

	if (result == CUBEOBC_ERROR_OK)
	{
		// Request file info telemetry
		result = tctlmCubeComputerControlProgram8_getFileInfo(endpoint, info);
	}

	return result;
}

ErrorCode cubeObc_cubeComputer_imagePollForState(TypeDef_TctlmEndpoint *endpoint,
												 TctlmCubeComputerCommon3_ImageTransferState state,
												 U32 backoff, U32 timeout,
												 TctlmCubeComputerCommon3_ImageTransferStatus *status)
{
	ErrorCode result;
	Boolean done = FALSE;
	U16 backoffTotal = 0u; // How long we have waited in total for initialization to complete

	do
	{
		result = tctlmCubeComputerCommon3_getImageTransferStatus(endpoint, status);

		if (result == CUBEOBC_ERROR_OK)
		{
			done = ((status->state == state) || (status->errorCode != CUBEOBC_ERROR_OK));

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

ErrorCode cubeObc_cubeComputer_imageGetInfoFirstLast(TypeDef_TctlmEndpoint *endpoint,
													 TctlmCubeComputerCommon3_ImageFileInfo *firstInfo,
													 TctlmCubeComputerCommon3_ImageFileInfo *lastInfo)
{
	if ((endpoint == NULL) || (firstInfo == NULL) || (lastInfo == NULL))
		return CUBEOBC_ERROR_NULLPTR;

	ErrorCode result;
	TctlmCubeComputerCommon3_ImageTransferSetup setup;

	ZERO_VAR(*firstInfo);
	ZERO_VAR(*lastInfo);
	ZERO_VAR(setup);

	setup.opCode = TCTLM_CUBE_COMPUTER_COMMON_3__INFO_RESET;

	result = tctlmCubeComputerCommon3_setImageTransferSetup(endpoint, &setup);

	if (result == CUBEOBC_ERROR_OK)
	{
		Boolean done = FALSE;

		do
		{
			TctlmCubeComputerCommon3_ImageFileInfo info;
			U32 retry = 10u;

			do
			{
				// Wait for info of next file to be populated
				cubeObc_time_delay(50u);

				result = tctlmCubeComputerCommon3_getImageFileInfo(endpoint, &info);

				retry--;
			}
			while ((result == CUBEOBC_ERROR_TCTLM_BUSY) && (retry > 0u));

			if (result == CUBEOBC_ERROR_OK)
			{
				if (info.isValid == TRUE)
				{
					if (info.first == TRUE)
					{
						MEMCPY((U8 *)firstInfo, (U8 *)&info, sizeof(info));
					}

					if (info.last == TRUE)
					{
						MEMCPY((U8 *)lastInfo, (U8 *)&info, sizeof(info));

						done = TRUE;
					}
				}
				else
				{
					done = TRUE;
				}
			}
			else
			{
				done = TRUE;
			}
		}
		while (done == FALSE);
	}

	return result;
}

ErrorCode cubeObc_cubeComputer_imageGetInfo(TypeDef_TctlmEndpoint *endpoint,
											U32 fileHandle, TctlmCubeComputerCommon3_ImageFileInfo *info)
{
	if ((endpoint == NULL) || (info == NULL))
		return CUBEOBC_ERROR_NULLPTR;

	ErrorCode result;
	TctlmCubeComputerCommon3_ImageTransferSetup setup;

	ZERO_VAR(*info);
	ZERO_VAR(setup);

	setup.opCode = TCTLM_CUBE_COMPUTER_COMMON_3__INFO_RESET;

	result = tctlmCubeComputerCommon3_setImageTransferSetup(endpoint, &setup);

	if (result == CUBEOBC_ERROR_OK)
	{
		Boolean done = FALSE;
		Boolean found = FALSE;

		do
		{
			TctlmCubeComputerCommon3_ImageFileInfo imgInfo;
			U32 retry = 10u;

			do
			{
				// Wait for info of next file to be populated
				cubeObc_time_delay(50u);

				result = tctlmCubeComputerCommon3_getImageFileInfo(endpoint, &imgInfo);

				retry--;
			}
			while ((result == CUBEOBC_ERROR_TCTLM_BUSY) && (retry > 0u));

			if (result == CUBEOBC_ERROR_OK)
			{
				if (imgInfo.isValid == TRUE)
				{
					if (imgInfo.fileHandle == fileHandle)
					{
						MEMCPY((U8 *)info, (U8 *)&imgInfo, sizeof(imgInfo));

						done = TRUE;
						found = TRUE;
					}

					done = imgInfo.last;
				}
				else
				{
					done = TRUE;
				}
			}
			else
			{
				done = TRUE;
			}
		}
		while (done == FALSE);

		if ((result == CUBEOBC_ERROR_OK) && (found == FALSE))
		{
			result = CUBEOBC_ERROR_EXIST;
		}
	}

	return result;
}

ErrorCode cubeObc_cubeComputer_imageCaptureStore(TypeDef_TctlmEndpoint *endpoint,
												 TctlmCommonFrameworkEnums_AbstractNode nodeType,
												 TctlmCubeComputerCommon3_ImageTransferStatus *status)
{
	if ((endpoint == NULL) || (status == NULL))
		return CUBEOBC_ERROR_NULLPTR;

	ErrorCode result;
	TctlmCubeComputerCommon3_ImageTransferSetup setup;

	ZERO_VAR(*status);
	ZERO_VAR(setup);

	setup.opCode = TCTLM_CUBE_COMPUTER_COMMON_3__CAPTURE_STORE;
	setup.nodeType = nodeType;

	result = tctlmCubeComputerCommon3_setImageTransferSetup(endpoint, &setup);

	if (result == CUBEOBC_ERROR_OK)
	{
		// Check that image store has started
		result = cubeObc_cubeComputer_imagePollForState(endpoint, TCTLM_CUBE_COMPUTER_COMMON_3__STATE_BUSY_STORE, 10u, 5000u, status);

		if ((result == CUBEOBC_ERROR_OK) &&
			(status->errorCode != CUBEOBC_ERROR_OK))
		{
			// Requests for <ImageTransferStatus> were successful, but there was an internal error
			// The caller should inspect the returned "status" for details of the error
			result = CUBEOBC_ERROR_IMG;
		}

		if (result == CUBEOBC_ERROR_OK)
		{
			// Wait for image store to complete
			result = cubeObc_cubeComputer_imagePollForState(endpoint, TCTLM_CUBE_COMPUTER_COMMON_3__STATE_IDLE, 50u, 120000u, status);

			if ((result == CUBEOBC_ERROR_OK) &&
				(status->errorCode != CUBEOBC_ERROR_OK))
			{
				// Requests for <ImageTransferStatus> were successful, but there was an internal error
				// The caller should inspect the returned "status" for details of the error
				result = CUBEOBC_ERROR_IMG;
			}
		}
	}

	return result;
}

ErrorCode cubeObc_cubeComputer_imageDownloadDirect(TypeDef_TctlmEndpoint *endpoint,
												   TctlmCommonFrameworkEnums_AbstractNode nodeType, void *userData,
												   TctlmCubeComputerCommon3_ImageTransferStatus *status)
{
	if ((endpoint == NULL) || (status == NULL))
		return CUBEOBC_ERROR_NULLPTR;

	ErrorCode result;
	TctlmCubeComputerCommon3_ImageTransferSetup setup;

	ZERO_VAR(*status);
	ZERO_VAR(setup);

	setup.opCode = TCTLM_CUBE_COMPUTER_COMMON_3__CAPTURE_DOWNLOAD;
	setup.nodeType = nodeType;

	result = tctlmCubeComputerCommon3_setImageTransferSetup(endpoint, &setup);

	if (result == CUBEOBC_ERROR_OK)
	{
		result = cubeObc_cubeComputer_imagePollForState(endpoint, TCTLM_CUBE_COMPUTER_COMMON_3__STATE_BUSY_DOWNLOAD, 50u, 5000u, status);

		if ((result == CUBEOBC_ERROR_OK) &&
			(status->errorCode != CUBEOBC_ERROR_OK))
		{
			// Requests for <ImageTransferStatus> were successful, but there was an internal error
			// The caller should inspect the returned "status" for details of the error
			result = CUBEOBC_ERROR_IMG;
		}
	}

	if (result == CUBEOBC_ERROR_OK)
	{
		result = cubeObc_bulkDataTransfer_download(endpoint, userData);

		// Request status one final time after the upload to capture any errors that may have occurred
		(void)tctlmCubeComputerCommon3_getImageTransferStatus(endpoint, status);
	}

	return result;
}

ErrorCode cubeObc_cubeComputer_imageDownloadStored(TypeDef_TctlmEndpoint *endpoint,
												   U32 fileHandle, void *userData,
												   TctlmCubeComputerCommon3_ImageTransferStatus *status)
{
	if ((endpoint == NULL) || (status == NULL))
		return CUBEOBC_ERROR_NULLPTR;

	ErrorCode result;
	TctlmCubeComputerCommon3_ImageTransferSetup setup;

	ZERO_VAR(*status);
	ZERO_VAR(setup);

	setup.opCode = TCTLM_CUBE_COMPUTER_COMMON_3__DOWNLOAD;
	setup.fileHandle = fileHandle;

	result = tctlmCubeComputerCommon3_setImageTransferSetup(endpoint, &setup);

	if (result == CUBEOBC_ERROR_OK)
	{
		result = cubeObc_cubeComputer_imagePollForState(endpoint, TCTLM_CUBE_COMPUTER_COMMON_3__STATE_BUSY_DOWNLOAD, 50u, 5000u, status);

		if ((result == CUBEOBC_ERROR_OK) &&
			(status->errorCode != CUBEOBC_ERROR_OK))
		{
			// Requests for <ImageTransferStatus> were successful, but there was an internal error
			// The caller should inspect the returned "status" for details of the error
			result = CUBEOBC_ERROR_IMG;
		}
	}

	if (result == CUBEOBC_ERROR_OK)
	{
		result = cubeObc_bulkDataTransfer_download(endpoint, userData);

		// Request status one final time after the upload to capture any errors that may have occurred
		(void)tctlmCubeComputerCommon3_getImageTransferStatus(endpoint, status);
	}

	return result;
}

ErrorCode cubeObc_cubeComputer_eventPollForState(TypeDef_TctlmEndpoint *endpoint,
												 TctlmCubeComputerCommon3_EventReadQueueState state,
												 U32 backoff, U32 timeout,
												 TctlmCubeComputerCommon3_EventLogStatus *status)
{
	ErrorCode result;
	Boolean done = FALSE;
	U16 backoffTotal = 0u; // How long we have waited in total for initialization to complete

	do
	{
		result = tctlmCubeComputerCommon3_getEventLogStatus(endpoint, status);

		if (result == CUBEOBC_ERROR_OK)
		{
			done = (status->readQueueState == state);

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

ErrorCode cubeObc_cubeComputer_eventDownload(TypeDef_TctlmEndpoint *endpoint,
											 TctlmCubeComputerCommon3_EventLogFilterTransferSetup *setup, void *userData,
											 TctlmCubeComputerCommon3_EventLogStatus *status)
{
	if ((endpoint == NULL) || (status == NULL))
		return CUBEOBC_ERROR_NULLPTR;

	ErrorCode result;

	ZERO_VAR(*status);

	result = tctlmCubeComputerCommon3_setEventLogFilterTransferSetup(endpoint, setup);

	if (result == CUBEOBC_ERROR_OK)
	{
		result = cubeObc_cubeComputer_eventPollForState(endpoint, TCTLM_CUBE_COMPUTER_COMMON_3__EVT_READ_QDOWNLOAD,
														100u, 10000u, status);
	}

	if (result == CUBEOBC_ERROR_OK)
	{
		result = cubeObc_bulkDataTransfer_download(endpoint, userData);
	}

	return result;
}

ErrorCode cubeObc_cubeComputer_tlmIncludeLogId(U8 logId, TctlmCubeComputerCommon3_TelemetryLogTransferSetup *setup)
{
	U8 arrInd = (U8)(logId / 8);
	U8 bitPos = (U8)(logId % 8);
	U8 mask = (U8)(0b1 << bitPos);

	if (arrInd >= sizeof(setup->logIdBitmask))
		return CUBEOBC_ERROR_PARAM;

	setup->logIdBitmask[arrInd] |= mask;

	return CUBEOBC_ERROR_OK;
}

ErrorCode cubeObc_cubeComputer_tlmPollForState(TypeDef_TctlmEndpoint *endpoint,
											   TctlmCubeComputerCommon3_TlmLogReadQueueState state,
											   U32 backoff, U32 timeout,
											   TctlmCubeComputerCommon3_TelemtryLogStatus *status)
{
	ErrorCode result;
	Boolean done = FALSE;
	U16 backoffTotal = 0u; // How long we have waited in total for initialization to complete

	do
	{
		result = tctlmCubeComputerCommon3_getTelemtryLogStatus(endpoint, status);

		if (result == CUBEOBC_ERROR_OK)
		{
			done = (status->readQueueState == state);

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

ErrorCode cubeObc_cubeComputer_tlmDownload(TypeDef_TctlmEndpoint *endpoint,
										   TctlmCubeComputerCommon3_TelemetryLogTransferSetup *setup, void *userData,
										   TctlmCubeComputerCommon3_TelemtryLogStatus *status)
{
	if ((endpoint == NULL) || (status == NULL))
		return CUBEOBC_ERROR_NULLPTR;

	ErrorCode result;

	ZERO_VAR(*status);

	result = tctlmCubeComputerCommon3_setTelemetryLogTransferSetup(endpoint, setup);

	if (result == CUBEOBC_ERROR_OK)
	{
		result = cubeObc_cubeComputer_tlmPollForState(endpoint, TCTLM_CUBE_COMPUTER_COMMON_3__TLM_READ_QDOWNLOAD,
													  100u, 10000u, status);
	}

	if (result == CUBEOBC_ERROR_OK)
	{
		result = cubeObc_bulkDataTransfer_download(endpoint, userData);
	}

	return result;
}

/*** end of file ***/
