/**
 ******************************************************************************
 * \file    hCloudClient.h
 * \author  Husarion team
 * \version V0.9
 * \date    2-June-2014
 * \brief   Provides access to the online platform.
 ******************************************************************************
 * \details This file is part of hCloudClient library
 *
 * \copyright Copyright (C) 2014 Husarion Sp. z o.o. <husarion.com> - All Rights Reserved.
 * Unauthorized copying of this file and the hFramework library,
 * via any medium is strictly prohibited. Proprietary and confidential.
 *
 * \attention
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
 * INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
 * PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
 * HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
 * OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
 * SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 ******************************************************************************
 */

#ifndef __HCLOUDCLIENT_H__
#define __HCLOUDCLIENT_H__

#include <hFramework.h>

#include <hQueue.h>
#include <string.h>
#include <stdio.h>

/**
 * @brief hCloudClient
 */
namespace hCloudClient
{

enum class PacketType {
	PLATFORM, LOCAL, AUTH
};

/**
 * @brief Enum that contains key codes.
 */
enum class KeyCode {
	Backspace = 8,
	Tab = 9,
	Enter = 13,
	Shift = 16,
	Ctrl = 17,
	Alt = 18,
	PauseBreak = 19,
	CapsLock = 20,
	Esc = 27,
	Space = 32,
	PageUp = 33,
	PageDown = 34,
	End = 35,
	Home = 36,
	Left = 37,
	Up = 38,
	Right = 39,
	Down = 40,
	Insert = 45,
	Delete = 46,
	Key_0 = 48,
	Key_1 = 49,
	Key_2 = 50,
	Key_3 = 51,
	Key_4 = 52,
	Key_5 = 53,
	Key_6 = 54,
	Key_7 = 55,
	Key_8 = 56,
	Key_9 = 57,
	Key_A = 65,
	Key_B = 66,
	Key_C = 67,
	Key_D = 68,
	Key_E = 69,
	Key_F = 70,
	Key_G = 71,
	Key_H = 72,
	Key_I = 73,
	Key_J = 74,
	Key_K = 75,
	Key_L = 76,
	Key_M = 77,
	Key_N = 78,
	Key_O = 79,
	Key_P = 80,
	Key_Q = 81,
	Key_R = 82,
	Key_S = 83,
	Key_T = 84,
	Key_U = 85,
	Key_V = 86,
	Key_W = 87,
	Key_X = 88,
	Key_Y = 89,
	Key_Z = 90,
	LeftWinkey = 91,
	RightWinkey = 92,
	Select = 93,
	Numpad_0 = 96,
	Numpad_1 = 97,
	Numpad_2 = 98,
	Numpad_3 = 99,
	Numpad_4 = 100,
	Numpad_5 = 101,
	Numpad_6 = 102,
	Numpad_7 = 103,
	Numpad_8 = 104,
	Numpad_9 = 105,
	NumpadStar = 106,
	NumpadAdd = 107,
	NumpadMinus = 109,
	NumpadDot = 110,
	NumpadUnderscore = 111,
	F1 = 112,
	F2 = 113,
	F3 = 114,
	F4 = 115,
	F5 = 116,
	F6 = 117,
	F7 = 118,
	F8 = 119,
	F9 = 120,
	F10 = 121,
	F11 = 122,
	F12 = 123,
	NumLock = 144,
	ScrollLock = 145,
	Semicolon = 186,
	Equals = 187,
	Comma = 188,
	Minus = 189,
	Dot = 190,
	Underscore = 191,
	Grave = 192,
	RightBracket = 219,
	Backslash = 220,
	LeftBracket = 221,
	Apostrope = 222,
	NUMBER_OF_KEYS,
};

class hCloudClient;

/**
 * @brief Class that represents local serial interface (available via /dev/ttyCORE2)
 */
class hLocalSerial : public hPrintfDev {
public:
	hLocalSerial(hCloudClient& client);

	int write(const void* data, int len, uint32_t timeout = 0xffffffff);
	int read(void* data, int len, uint32_t timeout = 0xffffffff);

	int vprintf(const char* format, va_list arg);

	uint32_t available();

private:
	hCloudClient& cloudClient;

	friend class hCloudClient;
};

/**
 * @brief Key event type.
 */
enum class KeyEventType {
	Pressed = 1,
	Released = 0,
};

/**
 * @brief Button event type.
 */
enum class ButtonEventType {
	Pressed = 3,
	Released = 2,
};

struct Resource {
	enum Type {
		WEBIDE, //!< File created in Web IDE
		URL, //!< Any URL
		STRING, //!< Resource data as a raw string
		SDCARD //!< File from SD card.
	};
	Type tag; //!< Resource type
	const char* data; //!< Resource name or data (in case of type Resource::STRING)

	Resource(Type tag, const char* data): tag(tag), data(data) {}
	Resource(const Resource& other) = default;
};

/**
 * Represents a widget ID.
 *
 * Can be compared using \c == operator.
 */
struct hId {
	char id[30];
	hId()
	{
		id[0] = 0;
	}

	/**
	 * Creates identifier from a string.
	 */
	hId(const char* id)
	{
		if (!id) {
			this->id[0] = 0;
		} else {
			strncpy(this->id, id, sizeof(this->id));
		}
	}

	operator bool() const
	{
		return id[0] != 0;
	}

	char* str()
	{
		return id;
	}
	const char* c_str() const
	{
		return id;
	}
};

inline bool operator==(hId a, hId b)
{
	return strcmp(a.id, b.id) == 0;
}

typedef void (*ConfigHandler)();
typedef void (*OnKeyEventHandler)(KeyEventType type, KeyCode code);
typedef void (*OnButtonEventHandler)(hId id, ButtonEventType type);
typedef void (*OnConsoleWriteHandler)(hId id, const char* msg);
typedef void (*OnValueChangeEventHandler)(hId id, const char* data);

enum class PlatformStatus {
	Connected, Disconnected
};

typedef void (*OnPlatformStatusChangedHandler)(PlatformStatus status);

class PlatformUi;
class hCloudClient;

/**
 * Base class for all widgets.
 */
struct _UiWidget {
	hId id; //!< widget ID

	_UiWidget(hId id): id(id) {}

	void setLocation(int x, int y); //!< set widget absolute location
	void setSize(int width, int height); //!< set widget absolute size

	virtual int getType() const = 0;
};

/**
 * Base class for widgets containing text, including UiButton and UiLabel
 */
struct _UiWidgetWithText: public _UiWidget {
	_UiWidgetWithText(hId id): _UiWidget(id) {}

	/**
	 * set widget text
	 */
	void setText(const char* format, ...) __attribute__((format(printf, 2, 3)));

	/**
	 * set widget HTML
	 */
	void setHtml(const char* format, ...) __attribute__((format(printf, 2, 3)));
};

/**
 * A label. Extends _UiWidgetWithText.
 */
struct UiLabel: public _UiWidgetWithText {
	UiLabel(hId id): _UiWidgetWithText(id) {}

	int getType() const;
};

/**
 * A button. Extends _UiWidgetWithText.
 */
struct UiButton: public _UiWidgetWithText {
	UiButton(hId id): _UiWidgetWithText(id) {}

	int getType() const;
};

/**
 * A progress bar.
 */
struct UiProgressBar: public _UiWidget {
	UiProgressBar(hId id): _UiWidget(id) {}

	int getType() const;
	void setValue(int value); //!< sets progress bar value in percent
};

/**
 * A slider that can be dragged by user.
 */
struct UiSlider: public _UiWidget {
	UiSlider(hId id): _UiWidget(id) {}

	int getType() const;
	void setValue(int value); //!< sets slider value in percent
};

/**
 * A checkbox that can be selected by user.
 */
struct UiCheckBox: public _UiWidgetWithText {
	UiCheckBox(hId id): _UiWidgetWithText(id) {}

	int getType() const;
	void setChecked(bool checked); //!< sets if checkbox is checked
};

/**
 * A radio button that can be selected by user.
 */
struct UiRadioButton: public _UiWidgetWithText {
	UiRadioButton(hId groupId, hId id);

	int getType() const;
	void setChecked(bool checked); //!< sets if radio button is checked
};

/**
 * A radio group of radio buttons.
 */
struct UiRadioGroup {
	hId groupId;

	UiRadioGroup(hId groupId): groupId(groupId) { }

	/**
	 * Inserts UiRadioButton widget.
	 */
	UiRadioButton radioButton(hId id)
	{
		return UiRadioButton(groupId, id);
	}
};

enum class InputMode {
	TEXT, NUMERIC
};

/**
 * A text input, modifiable by user. Extends _UiWidgetWithText.
 */
struct UiInput: public _UiWidgetWithText {
	UiInput(hId id): _UiWidgetWithText(id) {}

	int getType() const;
	void setMode(InputMode mode); //!< Sets input mode.
};

/**
 * A console. .
 */
struct UiConsole: public _UiWidget {
	UiConsole(hId id): _UiWidget(id) {}

	int getType() const;
	void setAsMainLog();
	/**
	 * Appends text to the console.
	 */
	void printf(const char* format, ...) __attribute__((format(printf, 2, 3)));
};

struct UiVideo: public _UiWidget {
	UiVideo(): _UiWidget("video") {}

	int getType() const;
	void enable();
	void disable();
	void enableReception();
	void disableReception();
	void enableTransmission();
	void disableTransmission();
};

/**
 * @brief Controls the user interface on the RoboCORE platform.
 */
class PlatformUi {
private:
	PlatformUi(const PlatformUi&) = delete;
	PlatformUi() = default;
	friend class hCloudClient;
	friend struct UiVideo;

	void setVideoMode(int dir, int enabled);

public:
	ConfigHandler configHandler = nullptr;
	OnKeyEventHandler onKeyEvent = nullptr;
	OnButtonEventHandler onButtonEvent = nullptr;
	OnValueChangeEventHandler onValueChangeEvent = nullptr;
	OnConsoleWriteHandler onConsoleWrite = nullptr;

	UiVideo video;

	/**
	 * Sets WebIDE project ID. All files will be loaded from this project.
	 *
	 * @param id project ID like \c "012cbe51bc331503". Use \c "none" if you don't want to
	 *           load files from Web IDE.
	 */
	void setProjectId(const char* id);

	/**
	 * Loads script specified by given Resource.
	 */
	void loadScript(Resource res);

	/**
	 * Loads HTML template specified by given Resource.
	 */
	void loadHtml(Resource res);

	/**
	 * Loads CSS style specified by given Resource.
	 */
	void loadStyle(Resource res);

	// Widgets

	/**
	 * Inserts UiLabel widget.
	 */
	UiLabel label(hId id)
	{
		return UiLabel(id);
	}

	/**
	 * Inserts UiButton widget.
	 */
	UiButton button(hId id)
	{
		return UiButton(id);
	}

	/**
	 * Inserts UiProgressBar widget.
	 */
	UiProgressBar progressBar(hId id)
	{
		return UiProgressBar(id);
	}

	/**
	 * Inserts UiSlider widget.
	 */
	UiSlider slider(hId id)
	{
		return UiSlider(id);
	}

	/**
	 * Inserts UiConsole widget.
	 */
	UiConsole console(hId id)
	{
		return UiConsole(id);
	}

	/**
	 * Inserts UiInput widget.
	 */
	UiInput input(hId id)
	{
		return UiInput(id);
	}

	/**
	 * Inserts UiCheckBox widget.
	 */
	UiCheckBox checkBox(hId id)
	{
		return UiCheckBox(id);
	}

	/**
	 * Returns UiRadioGroup collection.
	 */
	UiRadioGroup radioGroup(hId id)
	{
		return UiRadioGroup(id);
	}
};

/**
 * @brief Provides access to the Husarion Cloud.
 */
class hCloudClient : public hPrintfDev {
public:
	hCloudClient();

	/**
	   * @brief Initialize connection to the cloud via given stream device.
	   *
	   * This also redirects all logs to the Web IDE console. If you want to
	   * revert this, use sys::setLogDev(hPrintfDev*).
	   *
	 * @param dev - stream device, can be Serial or Usb
	 */
	void begin(hStreamDev* dev);

	void begin(hPacketDev* dev);

	PlatformUi ui;

#ifdef DOXYGEN
	/**
	 * @brief Write formatted string to the IDE console.
	 */
	int printf(const char* format, ...) __attribute__((format(printf, 2, 3)));
#endif

	/**
	 * @brief Write formatted string to the IDE console.
	 */
	int vprintf(const char* str, va_list arg);

	/**
	 * @brief Write data to the IDE console.
	 */
	int write(const void* data, int len, uint32_t timeout = 5000);

	int read(void* data, int len, uint32_t timeout = 5000);

#ifdef DOXYGEN
	/**
	 * @brief Read single character from the console.
	 */
	char getch();
#endif

	uint32_t available();

	PlatformStatus getStatus();

	void setPingInterval(uint32_t timeMs);

	OnPlatformStatusChangedHandler statusChanged;

	hLocalSerial LocalSerial;

	friend class hLocalSerial;
};

extern hCloudClient platform;
}

using namespace hCloudClient;

#endif //__HCLOUDCLIENT_H__
