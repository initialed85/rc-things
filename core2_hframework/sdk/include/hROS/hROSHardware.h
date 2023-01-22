#ifndef HROS_HARDWARE_H_
#define HROS_HARDWARE_H_
#include <hFramework.h>
#include <cassert>

class hROSHardware {
    hStreamDev* device = nullptr;
  public:
    void init() {}

    int read() {
        unsigned char b;
        int readN = device->read(&b, 1, 10);
        if (readN == 0) return -1;
        return b;
    }

    void write(uint8_t* data, int length) {
        device->write(data, length);
    }

    unsigned long time() {
        return sys.getRefTime(); // TODO
    }

    void initWithDevice(hStreamDev* device) {
        assert(this->device == nullptr);
        this->device = device;
    }
};

#endif
