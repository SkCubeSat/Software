/** @file cubeObc_cubeComputer.h
 *
 * @brief libCubeObc CubeComputer operation helpers header file
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

#ifndef CUBEOBC_CUBE_COMPUTER__H
#define CUBEOBC_CUBE_COMPUTER__H

#ifdef __cplusplus
extern "C" {
#endif

/***************************** SYSTEM INCLUDES *******************************/

#include <cubeObc/cubeObc.h>

/************************ DEPENDENT MODULE INCLUDES **************************/

#include "tctlmCubeComputerCommon3.h"
#include "tctlmCubeComputerControlProgram8.h"

/***************************** GLOBAL DEFINES ********************************/

/**************************** GLOBAL CONSTANTS *******************************/

/****************************** GLOBAL MACROS ********************************/

/***************************** GLOBAL TYPEDEFS *******************************/

/***************************** GLOBAL VARIABLES ******************************/

/***************************** GLOBAL FUNCTIONS ******************************/

/**
 * @brief Poll FileTransferStatus telemetry for a specified state
 *
 * @note This will return immediately if a single FileTransferStatus request fails
 *
 * @param[in]	endpoint	Comms endpoint
 * @param[in]	state		State to poll for
 * @param[in]	backoff		Backoff time between polls [ms]
 * @param[in]	timeout		Total time to wait for state [ms]
 * @param[out]	status		Last requested FileTransferStatus
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_cubeComputer_pollForFtpState(TypeDef_TctlmEndpoint *endpoint,
											   TctlmCubeComputerControlProgram8_FtpState state,
											   U32 backoff, U32 timeout,
											   TctlmCubeComputerControlProgram8_FileTransferStatus *status);

/**
 * @brief Poll FileTransferStatus telemetry for a specified upgrade state
 *
 * @note This will return immediately if a single FileTransferStatus request fails
 *
 * @param[in]	endpoint	Comms endpoint
 * @param[in]	state		Upgrade state to poll for
 * @param[in]	backoff		Backoff time between polls [ms]
 * @param[in]	timeout		Total time to wait for state [ms]
 * @param[out]	status		Last requested FileTransferStatus
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_cubeComputer_pollForFtpUpgradeState(TypeDef_TctlmEndpoint *endpoint,
													  TctlmCubeComputerControlProgram8_FtpUpgradeState state,
													  U32 backoff, U32 timeout,
													  TctlmCubeComputerControlProgram8_FileTransferStatus *status);

/**
 * @brief Upload a CubeSpace (.cs) file to control-program
 *
 * @param[in]	endpoint	Comms endpoint
 * @param[in]	size		The size of the CubeSpace file
 * @param[in]	userData	User specific data
 * @param[out]	status		The last requested FileTransferStatus telemetry for debugging if return is not CUBEOBC_ERROR_OK.
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_cubeComputer_uploadCubeSpaceFile(TypeDef_TctlmEndpoint *endpoint, U32 size, void *userData,
												   TctlmCubeComputerControlProgram8_FileTransferStatus *status);

/**
 * @brief Perform an upgrade via CubeComputer - of CubeComputer or a connected node
 *
 * @param[in]	endpoint	Comms endpoint
 * @param[in]	nodeType	The node type of the node to upgrade - in conjunction with serialInt
 * @param[in]	serialInt	The integer portion of the serial number of the node to upgrade - in conjunction with nodeType
 * @param[in]	program		The program to upgrade
 * @param[in]	forcePort	Force a node bootloader upgrade on a specific port - used if a node is not auto-discovered
 * @param[out]	status		Last requested TelemetryLogStatus
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_cubeComputer_upgrade(TypeDef_TctlmEndpoint *endpoint,
									   TctlmCommonFrameworkEnums_NodeType nodeType, U32 serialInt,
									   TctlmCommonFrameworkEnums_ProgramType program,
									   TctlmCubeComputerControlProgram8_NodePort forcePort,
									   TctlmCubeComputerControlProgram8_FileTransferStatus *status);

/**
 * @brief Request file information for a single file
 *
 * @param[in]	endpoint	Comms endpoint
 * @param[in]	file		The file to request information for
 * @param[out]	info		File information for file
 * @param[out]	status		The last requested FileTransferStatus telemetry for debugging if return is not CUBEOBC_ERROR_OK.
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_cubeComputer_getFileInfo(TypeDef_TctlmEndpoint *endpoint,
										   TctlmCubeComputerControlProgram8_FtpFiles file,
										   TctlmCubeComputerControlProgram8_FileInfo *info,
										   TctlmCubeComputerControlProgram8_FileTransferStatus *status);

/**
 * @brief Poll ImageTransferStatus for a specified state
 *
 * @note This will return immediately if a single ImageTransferStatus request fails
 *
 * @param[in]	endpoint	Comms endpoint
 * @param[in]	state		State to poll for
 * @param[in]	backoff		Backoff time between polls [ms]
 * @param[in]	timeout		Total time to wait for state [ms]
 * @param[out]	status		Last requested ImageTransferStatus
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_cubeComputer_imagePollForState(TypeDef_TctlmEndpoint *endpoint,
												 TctlmCubeComputerCommon3_ImageTransferState state,
												 U32 backoff, U32 timeout,
												 TctlmCubeComputerCommon3_ImageTransferStatus *status);

/**
 * @brief Get image file information for the first image and the last image in the log
 *
 * The image log is a circular buffer. i.e. The are no dedicated slots in storage for files and
 * therefore file handles are assigned incrementally as new files are stored.
 * When the log wraps, the file handle for the new image will still increment from the previously stored image.
 * This function allows the user to determine the valid range of file handles in the log.
 * This can then be used to get the file information for exact file handles using cubeObc_bootloader_imageGetInfo().
 * This process can be used to confirm an image has been stored or to target an image to download.
 *
 * @param[in]	endpoint	Comms endpoint
 * @param[out]	firstInfo	File information for the first image in the log
 * @param[out]	lastInfo	File information for the last image in the log
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_cubeComputer_imageGetInfoFirstLast(TypeDef_TctlmEndpoint *endpoint,
													 TctlmCubeComputerCommon3_ImageFileInfo *firstInfo,
													 TctlmCubeComputerCommon3_ImageFileInfo *lastInfo);

/**
 * @brief Get image file information for a specified file handle
 *
 * The image log is a circular buffer. i.e. The are no dedicated slots in storage for files and
 * therefore file handles are assigned incrementally as new files are stored.
 * When the log wraps, the file handle for the new image will still increment from the previously stored image.
 * This function requires that the target file handle is known.
 * cubeObc_bootloader_imageGetInfoFirstLast() can be used to get the current valid range of file handles.
 *
 * @param[in]	endpoint	Comms endpoint
 * @param[out]	fileHandle	File handle to get information for
 * @param[out]	info		File information for fileHandle
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_cubeComputer_imageGetInfo(TypeDef_TctlmEndpoint *endpoint,
											U32 fileHandle, TctlmCubeComputerCommon3_ImageFileInfo *info);

/**
 * @brief Capture an image from an optical sensor and store it on board CubeComputer
 *
 * @note This function will only return once the image is fully downloaded from the node and stored, or on error.
 *
 * @param[in]	endpoint	Comms endpoint
 * @param[in]	nodeType	Target node to capture image from
 * @param[out]	status		Last requested ImageTransferStatus
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_cubeComputer_imageCaptureStore(TypeDef_TctlmEndpoint *endpoint,
												 TctlmCommonFrameworkEnums_AbstractNode nodeType,
												 TctlmCubeComputerCommon3_ImageTransferStatus *status);

/**
 * @brief Capture an image from an optical sensor and download it to OBC directly without storing on board CubeComputer
 *
 * @param[in]	endpoint	Comms endpoint
 * @param[in]	nodeType	Target node to capture image from
 * @param[in]	userData	User specific data
 * @param[out]	status		Last requested ImageTransferStatus
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_cubeComputer_imageDownloadDirect(TypeDef_TctlmEndpoint *endpoint,
												   TctlmCommonFrameworkEnums_AbstractNode nodeType, void *userData,
												   TctlmCubeComputerCommon3_ImageTransferStatus *status);

/**
 * @brief Download an image stored on board CubeComputer
 *
 * @param[in]	endpoint	Comms endpoint
 * @param[in]	fileHandle	File handle of image to download
 * @param[in]	userData	User specific data
 * @param[out]	status		Last requested ImageTransferStatus
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_cubeComputer_imageDownloadStored(TypeDef_TctlmEndpoint *endpoint,
												   U32 fileHandle, void *userData,
												   TctlmCubeComputerCommon3_ImageTransferStatus *status);

/**
 * @brief Poll EventLogStatus for a specified readQ state
 *
 * @note This will return immediately if a single EventLogStatus request fails
 *
 * @param[in]	endpoint	Comms endpoint
 * @param[in]	state		State to poll for
 * @param[in]	backoff		Backoff time between polls [ms]
 * @param[in]	timeout		Total time to wait for state [ms]
 * @param[out]	status		Last requested EventLogStatus
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_cubeComputer_eventPollForState(TypeDef_TctlmEndpoint *endpoint,
												 TctlmCubeComputerCommon3_EventReadQueueState state,
												 U32 backoff, U32 timeout,
												 TctlmCubeComputerCommon3_EventLogStatus *status);

/**
 * @brief Download event log
 *
 * @param[in]	endpoint	Comms endpoint
 * @param[in]	setup		Filter options for events to download
 * @param[in]	userData	User specific data
 * @param[out]	status		Last requested EventLogStatus
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_cubeComputer_eventDownload(TypeDef_TctlmEndpoint *endpoint,
											 TctlmCubeComputerCommon3_EventLogFilterTransferSetup *setup, void *userData,
											 TctlmCubeComputerCommon3_EventLogStatus *status);

/**
 * @brief Modify setup log ID inclusion mask to include a specific log ID in download
 *
 * @param[in]		logId	Log ID to include
 * @param[in,out]	setup	Filter options for telemetries to download - only modifies log ID mask
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_cubeComputer_tlmIncludeLogId(U8 logId, TctlmCubeComputerCommon3_TelemetryLogTransferSetup *setup);

/**
 * @brief Poll TelemetryLogStatus for a specified readQ state
 *
 * @note This will return immediately if a single TelemetryLogStatus request fails
 *
 * @param[in]	endpoint	Comms endpoint
 * @param[in]	state		State to poll for
 * @param[in]	backoff		Backoff time between polls [ms]
 * @param[in]	timeout		Total time to wait for state [ms]
 * @param[out]	status		Last requested TelemetryLogStatus
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_cubeComputer_tlmPollForState(TypeDef_TctlmEndpoint *endpoint,
											   TctlmCubeComputerCommon3_TlmLogReadQueueState state,
											   U32 backoff, U32 timeout,
											   TctlmCubeComputerCommon3_TelemtryLogStatus *status);

/**
 * @brief Download telemetry log
 *
 * @param[in]	endpoint	Comms endpoint
 * @param[in]	setup		Filter options for telemetries to download
 * @param[in]	userData	User specific data
 * @param[out]	status		Last requested TelemetryLogStatus
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_cubeComputer_tlmDownload(TypeDef_TctlmEndpoint *endpoint,
										   TctlmCubeComputerCommon3_TelemetryLogTransferSetup *setup, void *userData,
										   TctlmCubeComputerCommon3_TelemtryLogStatus *status);

/*****************************************************************************/

#ifdef __cplusplus
}
#endif
#endif /* CUBEOBC_CUBE_COMPUTER__H */
