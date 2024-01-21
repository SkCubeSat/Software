#include <CanDriver.hpp>
#include <queue>
#include <chrono>

using sockcanpp::CanDriver;
using sockcanpp::CanId;
using sockcanpp::CanMessage;

void sendCanFrameExample() {
    CanDriver canDriver("vcan0", CAN_RAW, 0xd00d);
    CanMessage messageToSend(0 /*send with default ID*/, "8 bytes!" /* the data */);
    auto sentByteCount = canDriver.sendMessage(messageToSend, false);
    printf("Sent %d bytes via CAN!\n", sentByteCount);
}


// main function to test the sendCanFrameExample() function
int main() {
    sendCanFrameExample();
    // delay for 1 second
    std::this_thread::sleep_for(std::chrono::seconds(1));
    sendCanFrameExample();
    std::this_thread::sleep_for(std::chrono::seconds(1));
    sendCanFrameExample();
    std::this_thread::sleep_for(std::chrono::seconds(1));
    sendCanFrameExample();
    return 0;
}