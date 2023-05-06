#include "hFramework.h"
#include <string>
#include <sstream>

using namespace hFramework;

// TODO: readLine() can block forever if the other end doesn't send a newline
std::string readLine() {
    std::string data;

    char buf[1] = {'\x00'};

    for (int i = 0; i < 1024; i++) {
        // TODO: need to work timeout argument + return value
        RPi.read(buf, 1);

        if (buf[0] == '\x00') {
            continue;
        }

        if (buf[0] == '\r') {
            // TODO: as above
            RPi.read(buf, 1); // read off the '\n'
            break;
        }

        data += buf[0];
    }

    return data;
}

void splitString(const std::string &str, char delimiter, std::string &first, std::string &second) {
    std::size_t pos = str.find(delimiter);
    first = str.substr(0, pos);
    second = str.substr(pos + 1);
}

[[noreturn]] void hMain() {
    printf("%d\tsetting up USB serial for logging...", (int) hFramework::hSystem::getRefTime());
    Serial.init(115200, Parity::None, StopBits::One);
    hFramework::hSystem::setLogDev(&Serial);

    printf("%d\tsetting up Raspberry Pi facing UART for commands...", (int) hFramework::hSystem::getRefTime());
    RPi.init(115200, Parity::None, StopBits::One);

    uint32_t last_blink = 0;

    auto scaled_left = 0;
    auto scaled_right = 0;

    printf("%d\tentering main loop...", (int) hFramework::hSystem::getRefTime());
    for (;;) {
        auto time = (int) hFramework::hSystem::getRefTime();

        if (time - last_blink > 1000) {
            hLED1.toggle();
            last_blink = time;
        }

        // TODO: this is a safety risk- if the other side sends max throttles and then nothing else, this code
        //   will stay at max throttles forever
        std::string line = readLine();

        std::string rawLeft, rawRight;
        splitString(line, ',', rawLeft, rawRight);

        std::stringstream ss;

        float left;
        ss << rawLeft;
        ss >> left;
        ss.clear();

        float right;
        ss << rawRight;
        ss >> right;
        ss.clear();

        scaled_left = (int16_t) (left * 1000.0);
        scaled_right = (int16_t) (right * 1000.0);

        printf(
                "%d\tline = %s, left=%f, right=%f, scaled_left = %d, scaled_right = %d\r\n",
                (int) hFramework::hSystem::getRefTime(),
                line.c_str(),
                left,
                right,
                scaled_left,
                scaled_right
        );

        hMotA.setPower((int16_t) scaled_left);
        hMotB.setPower((int16_t) scaled_right);
    }
}
