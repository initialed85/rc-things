/**
 * Copyright (c) 2013-2017 Husarion Sp. z o.o.
 * Distributed under the MIT license.
 * For full terms see the file LICENSE.md.
 */

#ifndef HDOUBLEPID_H
#define HDOUBLEPID_H

#define PID_FLAG_HAS_MIN  0x01
#define PID_FLAG_HAS_MAX  0x02
#define PID_FLAG_HAS_IMIN 0x04
#define PID_FLAG_HAS_IMAX 0x08
#define PID_FLAG_HAS_LAST_ERROR 0x10
#define PID_FLAG_I_ENABLED 0x20

#include <cstdint>

namespace hFramework
{

class hDoublePID
{
private:
	float scale, Kp_pos, Ki_pos, Kd_pos, Kp_neg, Ki_neg, Kd_neg;
	float isum;
	float imin, imax, min, max;

	float lastError;

	uint8_t flags;

public:
	hDoublePID();
	hDoublePID(float Kp_pos, float Ki_pos, float Kd_pos, float Kp_neg, float Ki_neg, float Kd_neg);

	void setScale(float scale) { this->scale = scale; }
	void setCoeffs(float Kp_pos, float Ki_pos, float Kd_pos, float Kp_neg, float Ki_neg, float Kd_neg) { this->Kp_pos = Kp_pos; this->Ki_pos = Ki_pos; this->Kd_pos = Kd_pos; this->Kp_neg = Kp_neg; this->Ki_neg = Ki_neg; this->Kd_neg = Kd_neg; }
	void setKPup(float Kp) { this->Kp_pos = Kp; }
	void setKIup(float Ki) { this->Ki_pos = Ki; }
	void setKDup(float Kd) { this->Kd_pos = Kd; }
	void setKPdown(float Kp) { this->Kp_neg = Kp; }
	void setKIdown(float Ki) { this->Ki_neg = Ki; }
	void setKDdown(float Kd) { this->Kd_neg = Kd; }

	float getScale() const { return scale; }
	float getKPpos() const { return Kp_pos; }
	float getKIpos() const { return Ki_pos; }
	float getKDpos() const { return Kd_pos; }
	float getKPneg() const { return Kp_neg; }
	float getKIneg() const { return Ki_neg; }
	float getKDneg() const { return Kd_neg; }

	float getIsum() const { return isum; }
	float getLastError() const { return lastError; }

	void reset();

	void enableI() { flags |= PID_FLAG_I_ENABLED; }
	void disableI() { flags &= ~PID_FLAG_I_ENABLED; }

	void setRange(float range) { setRangeMin(-range); setRangeMax(range); }
	void setRange(float min, float max) { setRangeMin(min); setRangeMax(max); }
	void setRangeMin(float min);
	void setRangeMax(float max);
	void setIRange(float range) { setIRangeMin(-range); setIRangeMax(range); }
	void setIRange(float min, float max) { setIRangeMin(min); setIRangeMax(max); }
	void setIRangeMin(float min);
	void setIRangeMax(float max);

	float update(float error, int dt_ms);
};

}

#endif