#ifndef __MPU9250_H__
#define __MPU9250_H__

#include <hFramework.h>
#include <hMath.h>

#ifndef MPU9250_MULTIPLE_INSTANCES
#define MPU9250_MULTIPLE_INSTANCES
#endif
#define MPU9250_CLASSNAME MPU9250Class
#include <mpudmp.h>

namespace hModules
{

class MPU9250 : public MPU9250Class
{
public:
// #pragma pack(1)
	// struct Reading
	// {
		// int16_t ax, ay, az;
		// uint16_t temp;
		// int16_t gx, gy, gz;
		// uint8_t mag_st1;
		// int16_t mx, my, mz;
		// uint8_t mag_st2;

		// void print()
		// {
			// printf("a: %d %d %d\r\ng: %d %d %d\r\nm: %d %d %d\r\n", ax, ay, az, gx, gy, gz, mx, my, mz);
		// }
	// };
// #pragma pack()

	enum class GyroScale { DPS_250, DPS_500, DPS_1000, DPS_2000 };
	enum class Error { None, I2CProtocol };

	MPU9250(ISensor_i2c& sens);
	~MPU9250();

	// DMP
	bool initDMP();

	// Non-DMP
	bool init();
	bool enableInterrupt();
	bool disableInterrupt();
	bool setGyroScale(GyroScale scale);

	void waitForData();

	bool process();

	// bool setSelfTestEnabled(bool gyro, bool accel, bool magnet);
	// bool getSelfTestRegisters(uint8_t& gx, uint8_t& gy, uint8_t& gz, uint8_t& ax, uint8_t& ay, uint8_t& az);

	// Error getLastError() const { return error; }

	ISensor_i2c& getSensor() { return sens; }

	uint8_t* getBuffer() { return fifoBuffer; }

	const hQuaternion& getQuaternion() { return quat; }

private:
	ISensor_i2c& sens;
	uint8_t fifoBuffer[42];
	hQuaternion quat;
	bool dmp;
	// Error error;
};

}

#endif
