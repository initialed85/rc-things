#include <cstddef>
#include <cstdint>
#include "hFramework.h"
#include <iostream>
#include <cstdio>
#include <vector>

#ifndef MATRIXBUTTONS
#define MATRIXBUTTONS

class MatrixButtons
{
  private:
    unsigned int width;
    unsigned int height;
    std::vector<std::function<void(void)>> handler;
    std::vector<unsigned int> width_poz;
    std::vector<unsigned int> height_poz;

    std::vector<hGPIO> pins_width;
    std::vector<hGPIO> pins_height;

    bool enable;
    bool debaunce(int pin_num);
    unsigned int find(unsigned int t_width, unsigned int t_height);

  public:
    MatrixButtons(unsigned int t_width, unsigned int t_height);
    void init();
    void update();
    void addButon(unsigned int poz_width, unsigned int poz_height, std::function<void(void)> function);
    void addPinOnHeight(hGPIO pin);
    void addPinOnWidth(hGPIO pin);
};

#endif //MATRIXBUTTONS
