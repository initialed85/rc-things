/*
 * This file is part of hFramework
 *
 * Copyright (C) 2014 Husarion Sp. z o.o. <husarion.com> -  All Rights Reserved
 *
 * Unauthorized copying of this file and the hFramework library, via any medium
 * is strictly prohibited.
 * Proprietary and confidential.
*/

#ifndef __HSYSTEM_H__
#define __HSYSTEM_H__

#include <functional>

#include <hTypes.h>
#include <hStorage.h>
#include <hPrintfDev.h>

/**
 * @brief hFramework
 */
namespace hFramework
{
void hMainInit(); // forward declaration

namespace stm32
{

#define CONFIG_MAX_ROBOT_TASK_CNT 20
#define CONFIG_MAX_ROBOT_TIMER_CNT 20

class hMotor;
class hSensor;
class hLegoSensor;
class hStorage;
class hSystem;

typedef void* hTaskHandle;

class hTaskPimpl;
class hTimerPimpl;

class hTask
{
public:
	const char* getName();
	bool join(uint32_t timeout = 0xffffffff);
	bool isRunning();

private:
	hTaskPimpl* pimpl;

	friend class hSystem;
};

class hTimer
{
public:
	/**
	 * @brief Start timer.
	 */
	void start();
	void stop();
	bool isRunning();
	void setPeriod(uint32_t period);
	void free();

private:
	hTimerPimpl* pimpl;

	friend class hSystem;
};

#pragma pack(1)
class TRoboCOREHeader
{
public:
	uint8_t headerVersion, type;
	uint32_t version, id;
	uint8_t key[19];
	uint16_t checksum;

	bool isClear() const;
	bool isValid() const;
	bool isKeyValid() const;

	void calcChecksum();
};
#pragma pack()

/**
 * @brief Speficies timer mode
 */
enum class TimerMode
{
	OneShot, //!< Timer handler would run only once.
	Repeat //!< Timer handler would be run at specific interval.
};

/**
 * @brief Speficies start mode
 */
enum class TimerRun
{
	Immediately, //!< Timer will start immediately.
		Wait //!< Timer will not be started, a explicit call to hTimer::start would be needed.
};

/**
 * @brief RTOS (Real-Time operating system) interface
 */
class hSystem
{
	friend class hMotor;
	friend void hFramework::hMainInit();

private:
	void init();

	hSystem(const hSystem&) = delete;

public:
	hSystem(); //!< Initialize system object.

	/**
	 * @brief Create new task with parameter list.
	 * @param taskProc an TaskProcUserData argument
	 * @param param a void pointer to the object with parameters
	 * @param priority a priority of the task (the higher the number, the higher priority)
	 * @param stack a size of table dynamically allocated as stack for current task (single element of this table is uint32_t)
	 * @param desc a C-string with task description used in debugging
	 * @return the created task handle
	 */
	static hTask& taskCreate(const HandlerUserData& handler, void* param = 0, uint8_t priority = 2, uint32_t stack = 400, const char* desc = 0)
	{
		return taskCreate(AutoHandler(handler, param), priority, stack, desc);
	}

	/**
	 * @brief Create new task with parameter list.
	 * @param taskProc an TaskProc argument
	 * @param priority a priority of the task (the higher the number, the higher priority)
	 * @param stack a size of table dynamically allocated as stack for current task (single element of this table is uint32_t)
	 * @param desc a C-string with task description used in debugging
	 * @return the created task handle
	 */
	static hTask& taskCreate(const Handler& handler, uint8_t priority = 2, uint32_t stack = 400, const char* desc = 0)
	{
		return taskCreate(AutoHandler(handler), priority, stack, desc);
	}

	static hTimer& addTimer(uint32_t timeout, TimerMode mode, TimerRun startMode, const Handler& handler)
	{
		return addTimer(timeout, mode, startMode, AutoHandler(handler));
	}

	static hTimer& addTimer(uint32_t timeout, TimerMode mode, TimerRun startMode, const HandlerUserData& handler, void* userdata)
	{
		return addTimer(timeout, mode, startMode, AutoHandler(handler, userdata));
	}

	static hTimer& addTimeout(uint32_t timeout, const Handler& handler)
	{
		return addTimer(timeout, TimerMode::OneShot, TimerRun::Immediately, AutoHandler(handler));
	}
	static hTimer& addTimeout(uint32_t timeout, const HandlerUserData& handler, void* userdata)
	{
		return addTimer(timeout, TimerMode::OneShot, TimerRun::Immediately, AutoHandler(handler, userdata));
	}

	static hTimer& addInterval(uint32_t interval, const Handler& handler)
	{
		return addTimer(interval, TimerMode::Repeat, TimerRun::Immediately, AutoHandler(handler));
	}
	static hTimer& addInterval(uint32_t interval, const HandlerUserData& handler, void* userdata)
	{
		return addTimer(interval, TimerMode::Repeat, TimerRun::Immediately, AutoHandler(handler, userdata));
	}

	/**
	 * @brief Get timestamp from all time working timer with 1us resolution
	 * @return timer counter value is [us]
	 */
	static uint32_t getUsTimVal();

	/**
	 * @brief Delay the current task.
	 * @param delay milliseconds to delay
	 */
	static void delay(uint32_t delayMs);

	/**
	 * @brief Delay the current task.
	 * @param delay microseconds to delay (must be multiply of 500)
	 */
	static void delayUs(uint32_t delay);

	/**
	 * @brief Delay the current task.
	 * @param delay milliseconds to delay
	 *
	 * @code
	 * #include "hFramework.h"
	 * void hMain() {
	 *   uint32_t t = sys.getRefTime();
	 *   while(1) {
	 *     sys.delaySync(t,1000);
	 *     // ... do something that no one knows whether it takes 100ms, 95ms, 87ms or something.
	 *     //delayMsSync() generates respectively 900ms, 905ms, 913ms
	 *     //so this piece of code is executed exacly every 1s.
	 *   }
	 * }
	 * @endcode
	 */
	static void delaySync(uint32_t& refTime, uint32_t delay);

	// @cond
	__attribute__((deprecated))
	static void delay_ms(uint32_t _delay) { delay(_delay); }
	__attribute__((deprecated))
	static void delay_us(uint32_t _delay) { delayUs(_delay); }
	__attribute__((deprecated))
	static void delay_ms_sync(uint32_t& refTime, uint32_t delay)
	{
		delaySync(refTime, delay);
	}
	// @endcond

	/**
	 * @brief Get system time.
	 * @return system time in ms
	 */
	static uint32_t getRefTime();

	/**
	 * @brief Get the random number generated by hardware random number generator.
	 * @return generated random number
	 */
	static uint32_t getRandNr();

	/**
	 * @brief Get power supply voltage.
	 * @return power supply voltage
	 */
	static float getSupplyVoltage();

	/**
	 * @brief Get power supply voltage in millivolts.
	 * @return power supply voltage in millivolts.
	 */
	static uint32_t getSupplyVoltageMV();

	// @cond
	__attribute__((deprecated))
	static float getBatteryLevel() { return getSupplyVoltage(); }
	// @endcond

	/**
	 * @brief Start critical section.
	 *
	 * Prevent task switching. Critical section should be as short as possible.
	 */
	static void startCritical();

	/**
	 * @brief End critical section.
	 *
	 * Enable tasks swithing.
	 */
	static void endCritical();

	/**
	 * @brief Get list of tasks that are currently registered in the system.
	 * @param taskList pointer to table in which TaskList will be stored
	 * @return pointer to taskList
	 */
	static char* getTaskList(char* taskList);

	/**
	 * @brief Return handle of current task.
	 *
	 * @return current task handle
	 */
	static void* getThisTaskHandle();

	/**
	 * @brief Get statistics of all tasks.
	 * @param stats pointer to table in which Stats will be stored
	 * @return pointer to stats
	 */
	static char* getStats(char* stats);

	enum class SortMode { None, Name, Ticks };
	static void printStats(SortMode sortMode = SortMode::Name, hPrintfDev* dev = 0);

	/**
	 * @brief Dynamicaly allocate memory on the heap.
	 * @param size number of bytes to be dynamically allocated
	 * @return pointer to dynamically allocated memory
	 */
	static void* malloc(uint32_t size);

	/**
	 * @brief Free allocated memory.
	 * @param ptr pointer to dynamically allocated memory
	 */
	static void free(void * ptr);

	/**
	 * @brief Get unique device 6-octet length ID.
	 * @param id pointer to buffer to store unique id
	 */
	static void getUid(uint8_t* id);

	static void getRobocoreKey(uint8_t* key);

	/**
	 * @brief Perform microcontroller software reset.
	 */
	static void reset();

#if BOARD(ROBOCORE) || BOARD(CORE2)
	/**
	 * @brief Enable USB port ability to charge connected devices.
	 *
	 * Switches USB port to Charging Downstream Port (CDP) mode.
	 */
	static void enableUsbCharging();

	/**
	 * @brief Disable USB port ability to charge connected devices.
	 *
	 * Switches USB port to Standard Downstream Port (SDP) mode.
	 */
	static void disableUsbCharging();
#endif

	static void disableAutoWatchdog();
	static void enableAutoWatchdog();

	static void enableSyslog();
	static void disableSyslog();

	/**
	   * @brief Set default log device.
	   *
	   * Device set by this method will be used by printf and sys.log.
	   * Example:
	   * \code{.cpp}
	   * sys.setLogDev(&Serial);
	   * sys.setLogDev(&platform);
	   * sys.setLogDev(&devNull);
	   * \endcode
	 * @param dev device
	 */
	static void setLogDev(hPrintfDev* dev);

	static void setSysLogDev(hPrintfDev* dev);

	/**
	 * @brief Log formatted string.
	 *
	 * To set default log device use setLogDev.
	 * @param str pointer to null terminated format string
	 * @param ... - optional arguments
	 * @return number of written characters
	 */
	static int log(const char* format, ...);

	/**
	 * @brief Log formatted string.
	 *
	 * To set default log device use setLogDev.
	 * @param str pointer to null terminated format string
	 * @param arg arguments as a va_list
	 * @return number of written characters
	 */
	static int vlog(const char* format, va_list arg);

	static int syslog(const char* format, ...);
	static int vsyslog(const char* format, va_list arg);

	// static int putc(char c);
	// static int syslog_putc(char c);

	/**
	 * @brief Return local persistent storage.
	 * @return storage object
	 */
	static hStorage& getStorage();

	static int fail_log(const char* format, ...);
	static int fail_vlog(const char* format, va_list arg);

	static void idleTask();
	static void tickHook();

	static void fault_handler();

#if BOARD(ROBOCORE)
	static hLegoSensor& getSensor(int num);
#elif BOARD(CORE2) || BOARD(CORE2MINI)
	static hSensor& getSensor(int num);
#else
#  error no board
#endif
	static hMotor& getMotor(int num);

private:
	static hTimer& addTimer(uint32_t timeout, TimerMode mode, TimerRun startMode, const AutoHandler& handler);
	static hTask& taskCreate(const AutoHandler& handler, uint8_t priority, uint32_t stack, const char* desc);

	static void taskRunProc(void* p);
	static void timerRunProc(void* p);
};

/**
 * @brief Null device
 *
 * All data written to this device is discarded.
 */
class DevNull : public hPrintfDev
{
public:
	int printf(const char* str, ...) { return 0; }
	int vprintf(const char* str, va_list arg) { return 0; }
	int write(const void* data, int len, uint32_t timeout) { return len; }
	int read(void* data, int len, uint32_t timeout) { return 0; }
};

extern hSystem sys;
extern DevNull devNull;
extern hTask tasks[CONFIG_MAX_ROBOT_TASK_CNT];
extern hTimer timers[CONFIG_MAX_ROBOT_TIMER_CNT];

}
}

#endif /* __HSYSTEM_H__ */
