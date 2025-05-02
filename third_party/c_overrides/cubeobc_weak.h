#ifndef CUBEOBC_WEAK__H
#define CUBEOBC_WEAK__H

#include <cubeObc/cubeObc_typeDef.h>
#include <cubeObc/cubeObc_bulkDataTransfer.h>
#include <stdio.h>

/**
 * @brief Data used during download
 *
 * @note This is propogated as void *userData
 */
typedef struct DownloadDataStruct {
	FILE *filePtr;						/**< File pointer */
	S32 fileSeekIdx;					/**< File seek index for next frame to store */
	U16 lastFrameSize;					/**< Size of the last frame requested by the lib */
	U8 frameBuffer[BDT_MAX_FRAME_SIZE];	/**< Buffer for a single frame of data */
} DownloadData;

ErrorCode cubeObc_canIfc_rx(CanPacket *packet);

ErrorCode cubeObc_canIfc_tx(const CanPacket *packet);

ErrorCode cubeObc_bulkDataTransfer_getFrameBuffer(void *userData, uint8_t **frame, uint16_t size);

ErrorCode cubeObc_bulkDataTransfer_commitFrameBuffer(void *userData, uint8_t *frame, uint16_t size);


#endif