/** @file cubeObc_example_eventDownload.c
 *
 * @brief libCubeObc Example - how to download the event log from CubeComputer
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */
#include <stdio.h>

#include <cubeObc/cubeObc.h>

#if (USE_CSP == 1)
#include <cubeObc/cubeObc.h>
#include <csp/csp.h>
#include <csp/interfaces/csp_if_can.h>
#include <pthread.h>
#endif

#define CAN_DEVICE	"can0"	/**< Socketcan device */

#define DOWNLOAD_FILE "/path/to/your/file/cubeObc_events.evt" /**< Path to file to store events */

#define CAN_ADDR_CC	((U8)2u)	/**< CubeComputer CAN address */

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

DownloadData downloadData;	/**< Data used during the download */

#if (USE_CSP == 1)

#define CSP_SRC_PORT	((U8)11u)	/**< Source port used for CSP comms with CubeProduct */

pthread_t canRxThreadHandle;		/**< CAN Rx thread handle */
Boolean threadRun = TRUE;			/**< CAN Rx thread exit signal */
csp_iface_t iface;					/**< CSP interface data */
csp_can_interface_data_t ifdata;	/**< CSP CAN interface data */
csp_socket_t *sock = NULL;			/**< CSP connectionless socket */

/* Thread to read the CAN bus and push packets to CSP */
void *canRxThread(void *ptr)
{
	while (threadRun == TRUE)
	{
		ErrorCode result;
		CanPacket packet;

		result = cubeObc_can_rx(&packet);

		if (result == CUBEOBC_ERROR_OK)
		{
			(void)csp_can_rx(&iface, packet.canExtId, packet.canData, (U8)packet.canSize, NULL);
		}
	}

	return NULL;
}

/* CSP callback to transmit CAN packet */
PRIVATE int commsCspTx(void *driver_data, U32 id, CONST U8 *data, U8 dlc)
{
	ErrorCode result;
	ErrorCode cspReturn;
	CanPacket packet;

	packet.canSize = dlc;
	packet.canExtId = id;
	MEMCPY(packet.canData, data, dlc);

	result = cubeObc_can_tx(&packet);

	if (result != CUBEOBC_ERROR_OK)
	{
		cspReturn = CSP_ERR_TIMEDOUT;
	}
	else
	{
		cspReturn = CSP_ERR_NONE;
	}

	/*
	 * !!NOTE!!
	 *
	 * CubeComputer requires a 1 millisecond delay between CAN packets.
	 * The exact implementation of this delay is OBC specific since the implementation is required in a CSP callback.
	 */
	cubeObc_time_delay(1u);

	return cspReturn;
}

/* Initialize CSP */
PRIVATE int initializeCsp(csp_conf_t *cspConfig, U8 address, CubeObc__Version *version)
{
	Text versionText[16u];

	sprintf(versionText, "%d.%d.%d", version->vMajor, version->vMinor, version->vPatch);

	cspConfig->address = address;
	cspConfig->buffer_data_size = 1024u;
	cspConfig->buffers = 5u;
	cspConfig->conn_dfl_so = CSP_O_NONE;
	cspConfig->conn_max = 1u;
	cspConfig->conn_queue_length = 5u;
	cspConfig->fifo_length = 5u;
	cspConfig->hostname = "libcubeobc";
	cspConfig->model = "libcubeobc";
	cspConfig->port_max_bind = 62u;
	cspConfig->rdp_max_window = 20u;
	cspConfig->revision = versionText;

	if (csp_init(cspConfig) != CSP_ERR_NONE)
	{
		printf("CSP initialization failed\r\n");
		return -1;
	}

	iface.name = CAN_DEVICE;
	iface.interface_data = &ifdata;
	iface.driver_data = NULL;
	ifdata.tx_func = commsCspTx;

	if (csp_can_add_interface(&iface) != CSP_ERR_NONE)
	{
		printf("CSP IFC initialization failed\r\n");
		return -1;
	}

	if (csp_rtable_set(CSP_DEFAULT_ROUTE, 0U, &iface, CSP_NO_VIA_ADDRESS) != CSP_ERR_NONE)
	{
		printf("CSP routing table initialization failed\r\n");
		return -1;
	}

	sock = csp_socket(CSP_SO_CONN_LESS);
	if (csp_bind(sock, CSP_SRC_PORT) != CSP_ERR_NONE)
	{
		printf("CSP socket initialization failed\r\n");
		return -1;
	}

	if (csp_route_start_task(4096u, 1u) != CSP_ERR_NONE)
	{
		printf("CSP route task failed\r\n");
		return -1;
	}

	// Start CAN RX thread
	if (pthread_create(&canRxThreadHandle, NULL, canRxThread, NULL) != 0)
	{
		printf("Start CAN RX thread failed\r\n");
		return -1;
	}

	return 0;
}

/* WEAK OVERRIDE */
ErrorCode cubeObc_cspIfc_recvFrom(U8 port, U8 *data, U16 *dataSize, U32 timeout)
{
	ErrorCode result = CUBEOBC_ERROR_OK;
	csp_packet_t *packet = csp_recvfrom(sock, timeout);

	if (packet != NULL)
	{
		*dataSize = packet->length;
		MEMCPY(data, packet->data, packet->length);

		csp_buffer_free(packet);
	}
	else
	{
		result = CUBEOBC_ERROR_CSP_RECV_TIMEOUT;
	}

	return result;
}

/* WEAK OVERRIDE */
ErrorCode cubeObc_cspIfc_sendTo(U8 dst, U8 dstPort, U8 srcPort, U8 *data, U16 dataSize, U32 timeout)
{
	ErrorCode result = CUBEOBC_ERROR_OK;
	U8 prio = 0u;

	csp_packet_t *packet = csp_buffer_get(dataSize);

	if (packet != NULL)
	{
		packet->length = dataSize;
		MEMCPY(packet->data, data, dataSize);

		result = csp_sendto(prio, dst, dstPort, srcPort, CSP_O_NONE, packet, timeout);

		if (result != CUBEOBC_ERROR_OK)
		{
			// Must free buffer on failure
			csp_buffer_free(packet);
		}
	}
	else
	{
		result = CUBEOBC_ERROR_CSP_BUFFER_NONE;
	}

	return result;
}

#endif // USE_CSP

/* WEAK OVERRIDE */
ErrorCode cubeObc_canIfc_rx(CanPacket *packet)
{
	return cubeObc_can_rx(packet);
}

/* WEAK OVERRIDE */
ErrorCode cubeObc_canIfc_tx(CONST CanPacket *packet)
{
	return cubeObc_can_tx(packet);
}

/* WEAK OVERRIDE */
ErrorCode cubeObc_bulkDataTransfer_getFrameBuffer(void *userData, U8 **frame, U16 size)
{
	ErrorCode result = CUBEOBC_ERROR_OK;

	DownloadData *data = (DownloadData *)userData;

	// The bulk data transfer will write the frame in frameBuffer and then commit it
	*frame = data->frameBuffer;

	data->lastFrameSize = size;

	return result;
}

/* WEAK OVERRIDE */
ErrorCode cubeObc_bulkDataTransfer_commitFrameBuffer(void *userData, U8 *frame, U16 size)
{
	ErrorCode result = CUBEOBC_ERROR_OK;

	DownloadData *data = (DownloadData *)userData;

	if ((frame == data->frameBuffer) && (size == data->lastFrameSize))
	{
		fseek(data->filePtr, data->fileSeekIdx, SEEK_SET);

		U32 written = fwrite(data->frameBuffer, 1, size, data->filePtr);

		if (written != size)
		{
			result = CUBEOBC_ERROR_SIZE;
		}

		data->fileSeekIdx += size;
	}
	else
	{
		result = CUBEOBC_ERROR_COMMIT;
	}

	return result;
}

int main(void)
{
	ErrorCode result;
	TypeDef_TctlmEndpoint endpoint;
	CubeObc_Config config;
	CubeObc__Version version;
	CubeObc__Version sysVersion;

	config.hostAddress = 1u;

	endpoint.nodeType = TCTLM_COMMON_FRAMEWORK_ENUMS__NODE_TYPE_CUBE_COMPUTER;
	endpoint.type = TYPEDEF__COMMS_ENDPOINT_CAN;
	endpoint.proto = TYPEDEF__COMMS_PROTOCOL_CUBESPACE;
	endpoint.addr = CAN_ADDR_CC;
	endpoint.timeout = 500u;
	endpoint.passthrough = FALSE;

	cubeObc_init(&config);

	cubeObc_getVersion(&version);
	cubeObc_getSystemVersion(&sysVersion);

	printf("libCubeObc Version: %u.%u.%u\r\n", version.vMajor, version.vMinor, version.vPatch);
	printf("libCubeObc System Version: %u.%u.%u\r\n", sysVersion.vMajor, sysVersion.vMinor, sysVersion.vPatch);

	result = cubeObc_can_init(CAN_DEVICE);

	if (result != CUBEOBC_ERROR_OK)
	{
		printf("CAN initialization failed: %u\r\n", result);
		return -1;
	}

#if (USE_CSP == 1)

	printf("Using CSP Protocol\r\n");

	endpoint.proto = TYPEDEF__COMMS_PROTOCOL_CSP;
	endpoint.cspSrcPort = CSP_SRC_PORT;

	csp_conf_t cspConfig;

	if (initializeCsp(&cspConfig, config.hostAddress, &version) != 0)
	{
		return -1;
	}

#else // USE_CSP == 0

	printf("Using CubeSpace Protocol\r\n");

#endif // USE_CSP

	/* MAKE SURE WE ARE TALKING TO CUBECOMPUTER*/

	TctlmCommonFramework1_Identification identification;

	result = tctlmCommonFramework1_getIdentification(&endpoint, &identification);

	if (result != CUBEOBC_ERROR_OK)
	{
		printf("Failed to get identification: %u\r\n", result);
		return -1;
	}

	if (identification.nodeType != TCTLM_COMMON_FRAMEWORK_ENUMS__NODE_TYPE_LEGACY_CUBE_COMPUTER)
	{
		printf("Connected node is not CubeComputer!\r\n");
		return -1;
	}

	/* EVENT DOWNLOAD */

	printf("Downloading events to file: %s\r\n", DOWNLOAD_FILE);

	downloadData.filePtr = fopen(DOWNLOAD_FILE, "w");
	if (downloadData.filePtr == NULL)
	{
		printf("Failed to open file\r\n");
		return -1;
	}

	// We will write to the file frame by frame, starting from index 0
	downloadData.fileSeekIdx = 0;

	TctlmCubeComputerCommon3_EventLogStatus status;
	TctlmCubeComputerCommon3_EventLogFilterTransferSetup setup;

	MEMSET((U8 *)&setup, 0xFF, sizeof(setup)); // Include all classes and all sources

	// Now replace 0xFF in the fields we care about
	// We will request the last 500 events
	setup.filterType = TCTLM_CUBE_COMPUTER_COMMON_3__FILTER_LAST_X;
	setup.numEntries = 500u;
	// All other setup parameters are don't-care for this filter type

	result = cubeObc_cubeComputer_eventDownload(&endpoint, &setup, (void *)&downloadData, &status);

	if (result != CUBEOBC_ERROR_OK)
	{
		printf("Event download failed: %u\r\n", result);
		return -1;
	}

	printf("SUCCESS\r\n");

	fclose(downloadData.filePtr);

#if (USE_CSP == 1)

	threadRun = FALSE;
	pthread_join(canRxThreadHandle, NULL);

#endif

	return 0;
}
