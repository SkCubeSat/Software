#include "cubeObc.h"
#include "cubeObc_bulkDataTransfer.h"
#include "cubeobc_weak.h"

ErrorCode cubeObc_canIfc_rx(CanPacket *packet) {
    return cubeObc_can_rx(packet);
}

ErrorCode cubeObc_canIfc_tx(const CanPacket *packet) {
    return cubeObc_can_tx(packet);
}

ErrorCode cubeObc_bulkDataTransfer_getFrameBuffer(void *userData, uint8_t **frame, uint16_t size) {
    DownloadData *data = (DownloadData *)userData;
    *frame = data->frameBuffer;
    data->lastFrameSize = size;
    return CUBEOBC_ERROR_OK;
}

ErrorCode cubeObc_bulkDataTransfer_commitFrameBuffer(void *userData, uint8_t *frame, uint16_t size) {
    DownloadData *data = (DownloadData *)userData;

    if ((frame == data->frameBuffer) && (size == data->lastFrameSize)) {
        fseek(data->filePtr, data->fileSeekIdx, SEEK_SET);
        uint32_t written = fwrite(data->frameBuffer, 1, size, data->filePtr);
        if (written != size) return CUBEOBC_ERROR_SIZE;
        data->fileSeekIdx += size;
        return CUBEOBC_ERROR_OK;
    }

    return CUBEOBC_ERROR_COMMIT;
}



