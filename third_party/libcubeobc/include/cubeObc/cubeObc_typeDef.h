/** @file cubeObc_typeDef.h
 *
 * @brief Type Definitions for libCubeObc
 *
 * @par
 * COPYRIGHT NOTICE: (c) 2023 Cubespace ADCS All rights reserved.
 */

#ifndef CUBEOBC_TYPE_DEF__H
#define CUBEOBC_TYPE_DEF__H

#ifdef __cplusplus
extern "C" {
#endif

/***************************** SYSTEM INCLUDES *******************************/

#include <stdint.h>
#include <string.h>
#include <math.h>

#include "tctlmCommonFrameworkEnums.h"

/************************ DEPENDENT MODULE INCLUDES **************************/

/***************************** GLOBAL DEFINES ********************************/

#define PRIVATE static				/**< static */
#define CONST const					/**< const */
#define VOLATILE volatile			/**< volatile */
#define WEAK __attribute__((weak))	/**< weak */

#if (USE_TCTLM_PACKED==1)
#define TCTLM_PACKED __attribute__((packed))	/*!< TCTLM structs are packed */
#else
#define TCTLM_PACKED							/*!< TCTLM structs are not packed */
#endif

#define FALSE ((Boolean)0U)			/**< Boolean true */
#define TRUE  ((Boolean)1U)			/**< Boolean false */

/**
 * @brief CAN packet data payload in bytes
 */
#define CAN_DATA_BYTES   ((U32)8u)

/**************************** GLOBAL CONSTANTS *******************************/

/****************************** GLOBAL MACROS ********************************/

/**
 * @brief memset variable to all zero's
 */
#define ZERO_VAR(var)							\
		do										\
		{										\
			(void)memset((void *)&(var), 0, sizeof(var));	\
		}										\
		while (FALSE)

/**
 * @brief memcpy
 */
#define MEMCPY(dst, src, size)							\
		do												\
		{												\
			(void)memcpy(dst, src, size);				\
		}												\
		while (FALSE)

/**
 * @brief memset
 */
#define MEMSET(ptr, val, size)							\
		do												\
		{												\
			(void)memset(ptr, val, size);				\
		}												\
		while (FALSE)

/**
 * @brief CHECK_POW2 Assert if the constant is not a power of two
 *
 * We use a "statement expression" here. Why? .. Why not.
 *
 * @param[in] x Input value
 */
#define CHECK_POW2(x)  (__extension__({U32 input = x; (input != 0u) && ((input & (input - 1u)) == 0u); }))

/***************************** GLOBAL TYPEDEFS *******************************/

typedef _Bool Boolean;	/**< Boolean */
typedef uint8_t U8;		/**< unsigned int - 8 */
typedef int8_t S8;		/**< signed int - 8 */
typedef uint16_t U16;	/**< unsigned int - 16 */
typedef int16_t S16;	/**< signed int - 16 */
typedef uint32_t U32;	/**< unsigned int - 32 */
typedef int32_t S32;	/**< signed int - 32 */
typedef uint64_t U64;	/**< unsigned int - 64 */
typedef int64_t S64;	/**< signed int - 64 */
typedef float F32;		/**< float - 32 */
typedef double F64;		/**< float - 64 */
typedef char Text;		/**< char */

/**
 * @brief Error Code - see cubeObc_errorDef.h
 */
typedef int32_t ErrorCode;

/**
 * @brief CAN Structure
 */
typedef enum CanIdTypeEnum {
	CAN_ID_TYPE_EXTENDED = 0,	/**< Extended ID */
	CAN_ID_TYPE_STANDARD,		/**< Standard ID */
} CanIdType;

/**
 * @brief CAN Structure
 */
typedef struct CanPacketStruct {
	U32 canExtId;				/**< Extended 29bit ID field */
	U32 canStdId;				/**< Standard 11bit ID field */
	CanIdType idType;			/**< CAN ID type (standard or extended) */
	U32 canSize;				/**< Number of bytes used in packet */
	U8 canData[CAN_DATA_BYTES];	/**< Payload array */
} CanPacket;

/**
 * @brief Type of transport to use for reaching node
 */
typedef enum TypeDef_CommsEndpointTypeEnum {
	TYPEDEF__COMMS_ENDPOINT_CAN = 0,	/**< Use CAN Slave Bus */
	TYPEDEF__COMMS_ENDPOINT_I2C = 1,	/**< Use I2C Slave Bus */
	TYPEDEF__COMMS_ENDPOINT_UART = 2,	/**< Use UART Slave Bus */

	TYPEDEF__COMMS_ENDPOINT_MAX			/**< Max endpoints */
} TypeDef_CommsEndpointType;

/**
 * @brief Type of protocol to use over transport
 */
typedef enum TypeDef_CommsProtocolEnum {
	TYPEDEF__COMMS_PROTOCOL_CUBESPACE = 0,	/**< CubeSpace protocol */
	TYPEDEF__COMMS_PROTOCOL_CSP,			/**< Cubesat-Space-Protocol (CSP) - CAN endpoint only */
} TypeDef_CommsProtocol;

/**
 * @brief Endpoint type and address
 */
typedef struct TypeDef_TctlmEndpointStruct {
	TctlmCommonFrameworkEnums_NodeType nodeType;	/**< Node Type for endpoint */
	TypeDef_CommsEndpointType type;					/**< Endpoint type */
	TypeDef_CommsProtocol proto;					/**< Endpoint protocol */
    U32 addr;										/**< Endpoint address */
    U32 addrPass;									/**< Endpoint address for passthrough (CAN only) */
    U8 cspSrcPort;									/**< Source port to use if using CSP protocol */
    U32 timeout;									/**< Transaction timeout */
    Boolean passthrough;							/**< Signal passthrough transaction (UART only) */
} TypeDef_TctlmEndpoint;

/***************************** GLOBAL VARIABLES ******************************/

/***************************** GLOBAL FUNCTIONS ******************************/

#ifdef __cplusplus
}
#endif
#endif /* CUBEOBC_TYPE_DEF__H */
