#include <stdint.h>
#include <string.h>

#include <csp/csp.h>
#include <csp/csp_buffer.h>
#include <csp/csp_id.h>
#include <csp/csp_iflist.h>
#include <csp/csp_rtable.h>

#define RADSAT_CAPTURE_FRAME_MAX (CSP_BUFFER_SIZE + 8)

static uint8_t captured_frame[RADSAT_CAPTURE_FRAME_MAX];
static uint16_t captured_frame_len;
static uint8_t captured_i2c_addr;
static int captured_from_me;
static csp_id_t captured_id;

static int capture_nexthop(csp_iface_t * iface, uint16_t via, csp_packet_t * packet, int from_me) {
    (void)iface;

    csp_id_prepend(packet);

    captured_frame_len = packet->frame_length;
    if (captured_frame_len > RADSAT_CAPTURE_FRAME_MAX) {
        captured_frame_len = RADSAT_CAPTURE_FRAME_MAX;
    }
    memcpy(captured_frame, packet->frame_begin, captured_frame_len);

    captured_i2c_addr = ((via != CSP_NO_VIA_ADDRESS) ? via : packet->id.dst) & 0x7F;
    captured_from_me = from_me;
    captured_id = packet->id;

    csp_buffer_free(packet);
    return CSP_ERR_NONE;
}

static csp_iface_t capture_iface = {
    .name = "CAPTURE",
    .nexthop = capture_nexthop,
    .is_default = 1,
};

void radsat_csp_capture_iface_reset(void) {
    memset(captured_frame, 0, sizeof(captured_frame));
    captured_frame_len = 0;
    captured_i2c_addr = 0;
    captured_from_me = 0;
    memset(&captured_id, 0, sizeof(captured_id));
}

void radsat_csp_capture_iface_add(uint16_t addr) {
    capture_iface.addr = addr;
    capture_iface.netmask = 0;
    capture_iface.is_default = 1;
    csp_iflist_add(&capture_iface);
}

void radsat_csp_capture_iface_remove(void) {
    csp_iflist_remove(&capture_iface);
}

const uint8_t * radsat_csp_capture_frame_ptr(void) {
    return captured_frame;
}

uint16_t radsat_csp_capture_frame_len(void) {
    return captured_frame_len;
}

uint8_t radsat_csp_capture_i2c_addr(void) {
    return captured_i2c_addr;
}

int radsat_csp_capture_from_me(void) {
    return captured_from_me;
}

csp_id_t radsat_csp_capture_id(void) {
    return captured_id;
}
