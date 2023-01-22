#ifndef __SERVODRIVER_H__
#define __SERVODRIVER_H__

#include <hFramework.h>

/**
 * @brief hModules
 */
namespace hModules
{

class ServoDriver;

/**
 * @brief Provides interface to servo located on hExt Servo Driver module.
 */
class ServoDriver_Servo : public IServo
{
public:
	ServoDriver_Servo(ServoDriver& driver, int num);

	void setWidth(uint16_t widthUs);
	void setPeriod(uint16_t periodUs);

private:
	ServoDriver& driver;
	int num;

	ServoDriver_Servo(const ServoDriver_Servo&) = delete;
};

/**
 * @brief Provides interface to Servo Driver module.
 */
class ServoDriver
{
public:
#ifdef HAS_HEXT
	ServoDriver(hExtClass& ext, int address = 0);
#endif
	ServoDriver(ISerial& serial, int address = 0);

	void init(); //!< Initialize servo driver

	bool setWidth(int num, uint16_t widthUs);
	bool setPeriod(uint16_t periodUs) { period = periodUs; return update(); }

	bool update();

	void enableRetransmissions() { retransmissions = true; }
	void disableRetransmissions() { retransmissions = false; }

	bool enablePower() { powerEnabled = 1; return update(); }
	bool disablePower() { powerEnabled = 0; return update(); }

	void enableAutoUpdate() { autoUpdate = true; }
	void disableAutoUpdate() { autoUpdate = false; }

	//! Set power line voltage for all servos to about 4.8 [V].
	bool setPowerLow() { powerMode = 0; return update(); }
	//! Set power line voltage for all servos to about 6 [V].
	bool setPowerMedium() { powerMode = 1; return update(); }
	//! Set power line voltage for all servos to about 7.2 [V].
	bool setPowerHigh() { powerMode = 2; return update(); }
	//! Set power line voltage for all servos to about 8.2 [V].
	bool setPowerUltra() { powerMode = 3; return update(); }

	/**
	 * @brief Get servo by number.
	 * @param num - servo number
	 * @return reference to servo object
	 */
	ServoDriver_Servo& getServo(int num);

	ServoDriver_Servo s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12;

private:
#ifdef HAS_HEXT
	hExtClass* ext;
#endif
	ISerial& serial;
	bool initialized, autoUpdate, retransmissions;
	int address;
	uint16_t period;
	uint16_t width[12];
	uint8_t powerMode, powerEnabled;
	uint8_t pnum;
	hRecursiveMutex mutex;

	bool sendAndWaitForAck(void* data, int len);

	ServoDriver(const ServoDriver&) = delete;

	friend class ServoDriver_Servo;
};

}

#endif