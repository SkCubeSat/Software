/** @file cubeObc_example_passthrough.c
 *
 * @brief libCubeObc Example - how to perform passthrough communication via CubeComputer
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

#define USE_UART	(0)	/**< Select if UART test is done */
#define USE_CAN		(1) /**< Select if CAN test is done */

#define UART_DEVICE	"/dev/ttyUSB0"	/**< Terminal device */

#define CAN_DEVICE	"can0"	/**< Socketcan device */

#define CAN_ADDR_CC	((U8)2u)	/**< CubeComputer CAN address */

#define CAN_ADDR_CC_PASS	((U8)235u)	/**< CubeComputer CAN passthrough address (config item - this is the default) */

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

/**
 * @brief Perform a general passthrough test
 *
 * @param[in] endpoint Comms endpoint
 */
int performPassthrough(TypeDef_TctlmEndpoint *endpoint)
{
	ErrorCode result;

	// DISABLE PASSTHROUGH
	endpoint->passthrough = FALSE;

	TctlmCommonFramework1_Identification identification;

	result = tctlmCommonFramework1_getIdentification(endpoint, &identification); // To CubeComputer

	if (result != CUBEOBC_ERROR_OK)
	{
		printf("Failed to get identification: %u\r\n", result);
		return result;
	}

	if (identification.nodeType != TCTLM_COMMON_FRAMEWORK_ENUMS__NODE_TYPE_LEGACY_CUBE_COMPUTER)
	{
		printf("Connected node is not CubeComputer!\r\n");
		return result;
	}

	// Set the passthrough target

	result = tctlmCubeComputerCommon3_setPassThrough(endpoint, TCTLM_COMMON_FRAMEWORK_ENUMS__NODE_FSS_0); // To CubeComputer

	if (result != CUBEOBC_ERROR_OK)
	{
		printf("Failed to set the passthrough target: %u\r\n", result);
		return result;
	}

	// Power on the target node for passthrough

	TctlmCubeComputerCommon3_PowerState power;

	ZERO_VAR(power);

	// Power on for passthrough
	// Note that POWER_ON_PASS prevents the control-program from communicating with the node
	// However, it is still possible to perform passthrough communications while the control-program is using the node
	power.fss0Power = TCTLM_CUBE_COMPUTER_COMMON_3__POWER_ON_PASS;

	result = tctlmCubeComputerCommon3_setPowerState(endpoint, &power); // To CubeComputer

	if (result != CUBEOBC_ERROR_OK)
	{
		printf("Failed to power on node: %u\r\n", result);
		return result;
	}

	// Wait for boot
	cubeObc_time_delay(1000u);

	// ENABLE PASSTHROUGH
	endpoint->passthrough = TRUE;

	result = tctlmCommonFramework1_getIdentification(endpoint, &identification); // To Node

	if (result != CUBEOBC_ERROR_OK)
	{
		printf("Failed to get identification with passthrough: %u\r\n", result);
		return result;
	}

	// We use the abstract type to define the passthrough target, so,
	// to keep the example simple, we do not check for the exact expected node type,
	// which would require requesting the "ExpectedNodes" telemetry and extracting the mapping...
	// In the OBC this mapping should be hard coded and easy to look up.
	// In this example, we only make sure we get a different identification than that of CubeComputer.
	// The node type enum value is printed bellow for validation.

	if (identification.nodeType == TCTLM_COMMON_FRAMEWORK_ENUMS__NODE_TYPE_LEGACY_CUBE_COMPUTER)
	{
		printf("Passthrough communication did not work!\r\n");
		return result;
	}

	printf("Passthrough identification NodeType: %u\r\n", (U32)identification.nodeType);
	printf("Passthrough identification ProgramType: %u\r\n", (U32)identification.programType);

	/*
	 * The tctlmCommonFramework1_getIdentification() telemetry is the most simple example
	 * because it should return a different NodeType when using passthrough.
	 * However, it might not best illustrate how to use passthrough...
	 *
	 * As an example, the following telemetry could slot in here to set the CubeSense boresight via CubeComputer:
	 *
	 * result = tctlmCubeSenseControlProgram5_setCamBoresight(endpoint, &setVal);
	 *
	 * So we call a CubeSense API, the endpoint is setup to target CubeComputer.
	 * The telecommand data will be formatted to set the boresight such that CubeSense can parse it,
	 * but the protocol data is changed so that CubeComputer knows that it is a passthrough message.
	 *
	 * Note that the endpoint nodeType stays as CubeComputer when using passthrough, since the communication is still physically with CubeComputer.
	 */

	// DISABLE PASSTHROUGH
	endpoint->passthrough = FALSE;

	// Power off the target node

	power.fss0Power = TCTLM_CUBE_COMPUTER_COMMON_3__POWER_OFF;

	result = tctlmCubeComputerCommon3_setPowerState(endpoint, &power); // To CubeComputer

	// Allow power dissipation in case another iteration is performed
	cubeObc_time_delay(1000u);

	return result;
}

/**
 * @brief main - CubeSpace passthrough example
 */
int main(void)
{
	ErrorCode result;
	TypeDef_TctlmEndpoint endpoint;
	CubeObc_Config config;
	CubeObc__Version version;
	CubeObc__Version sysVersion;

	config.hostAddress = 1u;

	cubeObc_init(&config);

	cubeObc_getVersion(&version);
	cubeObc_getSystemVersion(&sysVersion);

	printf("libCubeObc Version: %u.%u.%u\r\n", version.vMajor, version.vMinor, version.vPatch);
	printf("libCubeObc System Version: %u.%u.%u\r\n", sysVersion.vMajor, sysVersion.vMinor, sysVersion.vPatch);

	/************************/
	/********* UART *********/
	/************************/

#if (USE_UART == 1)

	endpoint.nodeType = TCTLM_COMMON_FRAMEWORK_ENUMS__NODE_TYPE_CUBE_COMPUTER;
	endpoint.type = TYPEDEF__COMMS_ENDPOINT_UART;
	endpoint.timeout = 500u;

	result = cubeObc_uart_init(UART_DEVICE, 921600u);

	if (result != CUBEOBC_ERROR_OK)
	{
		printf("UART initialization failed: %u\r\n", result);
		return -1;
	}

	printf("\r\nPerforming passthrough over UART...\r\n");

	result = performPassthrough(&endpoint);

	if (result != CUBEOBC_ERROR_OK)
	{
		printf("UART passthrough failed: %u\r\n", result);
	}

#endif

	/************************/
	/********* CAN **********/
	/************************/

#if (USE_CAN == 1)

	endpoint.nodeType = TCTLM_COMMON_FRAMEWORK_ENUMS__NODE_TYPE_CUBE_COMPUTER;
	endpoint.type = TYPEDEF__COMMS_ENDPOINT_CAN;
	endpoint.proto = TYPEDEF__COMMS_PROTOCOL_CUBESPACE;
	endpoint.addr = CAN_ADDR_CC;
	endpoint.addrPass = CAN_ADDR_CC_PASS;
	endpoint.timeout = 500u;

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

	printf("\r\nPerforming passthrough over CAN...\r\n");

	result = performPassthrough(&endpoint);

	if (result != CUBEOBC_ERROR_OK)
	{
		printf("CAN passthrough failed: %u\r\n", result);
	}

#endif

	printf("\r\nDONE\r\n");

	return 0;
}
