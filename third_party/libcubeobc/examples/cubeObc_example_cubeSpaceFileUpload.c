/** @file cubeObc_example_cubeSpaceFileUpload.c
 *
 * @brief libCubeObc Example - how to upload a CubeSpace file to cube-computer control-program
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

#define TO_BOOTLOADER	(1)	/**< Select if the file should be uploaded to bootloader or CubeComputer control-program */

#define USE_UART		(0) /**< Select to use UART (1) or CAN (0) */

#define UART_DEVICE	"/dev/ttyUSB0"	/**< Terminal device */

#define CAN_DEVICE	"can0"		/**< CAN device */
#define CAN_ADDR_CC	((U8)2u)	/**< CubeComputer CAN address */

#define UPLOAD_FILE "/path/to/your/file/<file>.cs"	/**< CubeSpace file to upload - use a file from software bundle */

#if ((USE_UART == 1 ) && (USE_CSP == 1))
#error "CSP is not supported over UART"
#endif

/**
 * @brief Data used during upload
 *
 * @note This is propogated as void *userData
 */
typedef struct UploadDataStruct {
	FILE *filePtr;						/**< File pointer */
	S32 fileSeekIdx;					/**< File seek index for next frame to extract */
	U16 lastFrameSize;					/**< Size of the last frame requested by the lib */
	U8 frameBuffer[BDT_MAX_FRAME_SIZE];	/**< Buffer for a single frame of data */
} UploadData;

UploadData uploadData;	/**< Data used during upload */

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
ErrorCode cubeObc_uartIfc_rx(U8 *data, U32 size, U32 *sizeRead)
{
	return cubeObc_uart_rx(data, size, sizeRead);
}

/* WEAK OVERRIDE */
ErrorCode cubeObc_uartIfc_tx(CONST U8 *data, U32 size)
{
	return cubeObc_uart_tx(data, size);
}

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

	UploadData *data = (UploadData *)userData;

	fseek(data->filePtr, data->fileSeekIdx, SEEK_SET);

	U32 numRead = fread(data->frameBuffer, 1, size, data->filePtr);

	if (numRead != size)
	{
		result = CUBEOBC_ERROR_SIZE;
	}

	*frame = data->frameBuffer;

	data->lastFrameSize = size;

	return result;
}

/* WEAK OVERRIDE */
ErrorCode cubeObc_bulkDataTransfer_commitFrameBuffer(void *userData, U8 *frame, U16 size)
{
	ErrorCode result = CUBEOBC_ERROR_OK;

	UploadData *data = (UploadData *)userData;

	if ((frame == data->frameBuffer) && (size == data->lastFrameSize))
	{
		data->fileSeekIdx += size;
	}
	else
	{
		result = CUBEOBC_ERROR_COMMIT;
	}

	return result;
}

/**
 * @brief main - CubeSpace file upload example
 */
int main(void)
{
	ErrorCode result;
	TypeDef_TctlmEndpoint endpoint;
	CubeObc_Config config;
	CubeObc__Version version;
	CubeObc__Version sysVersion;

	config.hostAddress = 1u;

	endpoint.nodeType = TCTLM_COMMON_FRAMEWORK_ENUMS__NODE_TYPE_CUBE_COMPUTER;
	endpoint.proto = TYPEDEF__COMMS_PROTOCOL_CUBESPACE;
	endpoint.timeout = 500u;
	endpoint.passthrough = FALSE;

	cubeObc_init(&config);

	cubeObc_getVersion(&version);
	cubeObc_getSystemVersion(&sysVersion);

	printf("libCubeObc Version: %u.%u.%u\r\n", version.vMajor, version.vMinor, version.vPatch);
	printf("libCubeObc System Version: %u.%u.%u\r\n", sysVersion.vMajor, sysVersion.vMinor, sysVersion.vPatch);

#if (USE_UART == 1)

	printf("Using UART\r\n");

	endpoint.type = TYPEDEF__COMMS_ENDPOINT_UART;

	result = cubeObc_uart_init(UART_DEVICE, 921600u);

	if (result != CUBEOBC_ERROR_OK)
	{
		printf("UART initialization failed: %u\r\n", result);
		return -1;
	}

#else // USE_UART == 0

	printf("Using CAN\r\n");

	endpoint.type = TYPEDEF__COMMS_ENDPOINT_CAN;
	endpoint.addr = CAN_ADDR_CC;

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
#endif // USE_UART

	/* MAKE SURE THE CUBE-COMPUTER CONTROL-PROGRAM IS RUNNING */

	TctlmCommonFramework1_Identification identification;

	result = tctlmCommonFramework1_getIdentification(&endpoint, &identification);

	if (result != CUBEOBC_ERROR_OK)
	{
		printf("Failed to get identification: %u\r\n", result);
		return -1;
	}

#if (TO_BOOTLOADER == 1)

	if (identification.programType != TCTLM_COMMON_FRAMEWORK_ENUMS__PROGRAM_TYPE_BOOTLOADER)
	{
		printf("Reset to bootloader...\r\n");

		(void)tctlmCommonFramework1_setReset(&endpoint, TCTLM_COMMON_FRAMEWORK_1__SOFT);

		// Wait for boot
		cubeObc_time_delay(1000u);

		result = tctlmCommonFramework1_getIdentification(&endpoint, &identification);

		if (result != CUBEOBC_ERROR_OK)
		{
			printf("Failed to get identification after reset: %u\r\n", result);
			return -1;
		}

		if (identification.programType != TCTLM_COMMON_FRAMEWORK_ENUMS__PROGRAM_TYPE_BOOTLOADER)
		{
			printf("Failed to enter bootloader!\r\n");
			return -1;
		}

		printf("Halting to bootloader...\r\n");

		result = tctlmCubeCommonBaseBootloader5_setHalt(&endpoint);

		if (result != CUBEOBC_ERROR_OK)
		{
			printf("Failed to get halt bootloader: %u\r\n", result);
			return -1;
		}

		// Continue to upload
	}

#else // TO_BOOTLOADER

	if (identification.nodeType != TCTLM_COMMON_FRAMEWORK_ENUMS__NODE_TYPE_LEGACY_CUBE_COMPUTER)
	{
		printf("Connected node is not CubeComputer!\r\n");
		return -1;
	}

	if (identification.programType != TCTLM_COMMON_FRAMEWORK_ENUMS__PROGRAM_TYPE_CONTROL)
	{
		if (identification.programType == TCTLM_COMMON_FRAMEWORK_ENUMS__PROGRAM_TYPE_BOOTLOADER)
		{
			printf("Jumping to control-program...\r\n");

			// Tel the bootloader to jump to app
			result = tctlmCubeCommonBaseBootloader5_setJumpToDefaultApp(&endpoint);

			if (result != CUBEOBC_ERROR_OK)
			{
				printf("Failed to command bootloader to jump: %u\r\n", result);
				return -1;
			}

			// Wait for boot
			cubeObc_time_delay(1000u);

			// Wait for port validation/auto-discovery to complete
			result =  cubeObc_common_pollForBootState(&endpoint, TCTLM_COMMON_FRAMEWORK_1__APPLICATION_RUNNING, 500u, 10000u, FALSE);

			if (result != CUBEOBC_ERROR_OK)
			{
				printf("control-program failed to complete port validation/auto-discovery within 10 seconds: %u\r\n", result);
				return -1;
			}
		}
		else
		{
			printf("Connected program is not control-program or bootloader!\r\n");
			return -1;
		}
	}

#endif // TO_BOOTLOADER

	/* FILE UPLOAD */

	printf("Uploading file: %s\r\n", UPLOAD_FILE);

	uploadData.filePtr = fopen(UPLOAD_FILE, "r");
	if (uploadData.filePtr == NULL)
	{
		printf("Failed to open file\r\n");
		return -1;
	}
	fseek(uploadData.filePtr, 0, SEEK_END);
	S32 fsize = ftell(uploadData.filePtr);

	// We will read the file frame by frame, starting from index 0 from the start
	// The library will handle reading the meta data
	uploadData.fileSeekIdx = 0;

#if (TO_BOOTLOADER == 1)

	printf("Uploading to bootloader...\r\n");

	TctlmCubeCommonBaseBootloader5_Errors errors;

	result = cubeObc_bootloader_uploadCubeSpaceFile(&endpoint, (U32)fsize, (void *)&uploadData, &errors);

	if (result != CUBEOBC_ERROR_OK)
	{
		printf("File upload failed: %u\r\n", result);
		printf("errors.result = %u\r\n", errors.result);
		return -1;
	}

#else // TO_BOOTLOADER == 0

	printf("Uploading to control-program...\r\n");

	TctlmCubeComputerControlProgram8_FileTransferStatus status;

	result = cubeObc_cubeComputer_uploadCubeSpaceFile(&endpoint, (U32)fsize, (void *)&uploadData, &status);

	if (result != CUBEOBC_ERROR_OK)
	{
		printf("File upload failed: %u\r\n", result);
		printf("status.errorCode = %u\r\n", status.errorCode);
		return -1;
	}

#endif // TO_BOOTLOADER

	printf("SUCCESS\r\n");

	fclose(uploadData.filePtr);

#if (USE_CSP == 1)

	threadRun = FALSE;
	pthread_join(canRxThreadHandle, NULL);

#endif

	return 0;
}
