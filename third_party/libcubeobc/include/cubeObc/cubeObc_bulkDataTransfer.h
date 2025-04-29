/** @file cubeObc_bulkDataTransfer.h
 *
 * @brief libCubeObc Bulk Data Transfer Handler Header file
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

#ifndef CUBEOBC_BULK_DATA_TRANSFER__H
#define CUBEOBC_BULK_DATA_TRANSFER__H

#ifdef __cplusplus
extern "C" {
#endif

/***************************** SYSTEM INCLUDES *******************************/

#include <cubeObc/cubeObc.h>

/************************ DEPENDENT MODULE INCLUDES **************************/

/***************************** GLOBAL DEFINES ********************************/

/**
 * @brief The maximum frame size for Bulk Data TRansfer
 */
#define BDT_MAX_FRAME_SIZE	((U16)256u)

/**************************** GLOBAL CONSTANTS *******************************/

/****************************** GLOBAL MACROS ********************************/

/***************************** GLOBAL TYPEDEFS *******************************/

/***************************** GLOBAL VARIABLES ******************************/

/***************************** GLOBAL FUNCTIONS ******************************/

/**
 * @brief Download data using the Bulk Data Transfer protocol
 *
 * @note This should only be called after the required setup has been complete.
 *
 * @note This library provides helpers for all download operations that will handle the setup procedure
 *
 * @param[in]	endpoint	Comms endpoint
 * @param[in]	userData	User specific data - passed to cubeObc_bulkDataTransfer_getFrameBuffer()
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_bulkDataTransfer_download(TypeDef_TctlmEndpoint *endpoint, void *userData);

/**
 * @brief Upload data using the Bulk Data Transfer protocol
 *
 * @note This should only be called after the required setup has been complete.
 *
 * @note This library provides helpers for all upload operations that will handle the setup procedure
 *
 * @param[in]	endpoint	Comms endpoint
 * @param[in]	userData	User specific data - passed to cubeObc_bulkDataTransfer_getFrameBuffer()
 * @param[in]	size		Size of data - this must be known for uploads
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
ErrorCode cubeObc_bulkDataTransfer_upload(TypeDef_TctlmEndpoint *endpoint, void *userData, U32 size);

/**
 * @brief Get the next frame buffer for a given transfer context
 *
 * @note This is a WEAK definition and must be implemented by the OBC
 *
 * This library implementation of the Bulk Data Transfer protocol does not assume that the OBC implementation
 * has enough volatile memory to hold all of the data for the transfer.
 * Rather, frame buffers are requested from the OBC as they are needed during the transfer.
 * It is left to the OBC to correctly index the buffer as frames are requested.
 *
 * When downloading data, a frame will be requested from the device, then a frame will be requested from the OBC using this function.
 * The frame from the device is then copied in to the OBC provided frame buffer, then
 * cubeObc_bulkDataTransfer_commitFrameBuffer() is called,
 * which allows the OBC to update its buffer index, and potentially write the frame to non-volatile storage.
 *
 * When uploading data, a frame will be requested from the OBC.
 * At this point the OBC can read the frame from non-volatile storage, or simply index the next frame from a buffer.
 * The frame is then sent to the device, then
 * cubeObc_bulkDataTransfer_commitFrameBuffer() is called,
 * which allows the OBC to update the buffer index.
 *
 * @param[in]		userData	User specific data - the OBC can use this to know which buffer to index
 * @param[in,out]	frame		Pointer to next frame
 * @param[in]		size		Size of the frame
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
WEAK ErrorCode cubeObc_bulkDataTransfer_getFrameBuffer(void *userData, U8 **frame, U16 size);

/**
 * @brief Commit the frame buffer that was previously acquired using cubeObc_bulkDataTransfer_getFrameBuffer()
 *
 * @note This is a WEAK definition and must be implemented by the OBC
 *
 * @param[in]		userData	User specific data - the OBC can use this to know which buffer to index
 * @param[in,out]	frame		Pointer to next frame
 * @param[in]		size		Size of the frame
 *
 * @return CUBEOBC_ERROR_OK if successful
 */
WEAK ErrorCode cubeObc_bulkDataTransfer_commitFrameBuffer(void *userData, U8 *frame, U16 size);

/*****************************************************************************/

#ifdef __cplusplus
}
#endif
#endif /* CUBEOBC_BULK_DATA_TRANSFER__H */
