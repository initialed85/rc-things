#include "hFramework.h"
#include <cstdio>
#include <string>
#include <cmath>

using namespace hFramework;

std::string readLine() {
    std::string data;
    char buf[1] = {'\x00'};

    for (int i = 0; i < 1024; i++) {
        RPi.read(buf, 1);

        if (buf[0] == '\x00') {
            continue;
        }

        if (buf[0] == '\r') {
            RPi.read(buf, 1); // read off the '\n'
            break;
        }

        data += buf[0];
    }

    return data;
}

void splitString(std::string str, char delimiter, std::string &first, std::string &second) {
    std::size_t pos = str.find(delimiter);
    first = str.substr(0, pos);
    second = str.substr(pos + 1);
}

[[noreturn]] void hMain() {
    // setup the USB serial for logging
    Serial.init(115200, Parity::None, StopBits::One);
    hFramework::hSystem::setLogDev(&Serial);

    // setup the Raspberry Pi facing UART for commands
    RPi.init(115200, Parity::None, StopBits::One);

    uint32_t last_blink = 0;

    for (;;) {
        auto time = (uint32_t) hFramework::hSystem::getRefTime();

        if (time - last_blink > 1000) {
            hLED1.toggle();
            last_blink = time;
        }

        std::string line = readLine();
        std::string rawLeft, rawRight;
        splitString(line, ',', rawLeft, rawRight);

        float left = std::stof(rawLeft);
        float right = std::stof(rawRight);

        auto scaled_left = (int16_t) (left * 1000.0);
        auto scaled_right = (int16_t) (right * 1000.0);

        hMotA.setPower(scaled_left);
        hMotB.setPower(scaled_right);

        printf(
                "%lu\tleft = %f, right = %f, scaled_left = %d, scaled_right = %d\r\n",
                (uint32_t) hFramework::hSystem::getRefTime(),
                left,
                right,
                scaled_left,
                scaled_right);
    }
}
