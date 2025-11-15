#pragma once
#include <array>

class UAV {
private:
	int id;
	std::array<double, 3> pos;
	std::array<double, 3> vel;

public:
	UAV(int setid, double x, double y, double z) :
	id(setid), pos{x, y, z}, vel{0, 0, 0} {}

	void set_velocity(double x, double y, double z) {
		vel = {x, y, z};
	}
	void set_velx(double x) { vel[0] = x; }
	void set_vely(double y) { vel[0] = y; }
	void set_velz(double z) { vel[0] = z; }

	void update_position(double dt) {
		pos[0] += vel[0] * dt;
		pos[1] += vel[1] * dt;
		pos[2] += vel[2] * dt; 
	}

	std::array<double, 3> get_pos() const { return pos; }
	double get_x() const { return pos[0]; }
	double get_y() const { return pos[1]; }
	double get_z() const { return pos[2]; }

	std::array<double, 3> get_vel() const { return vel; }
	double get_velx() const { return vel[0]; }
	double get_vely() const { return vel[1]; }
	double get_velz() const { return vel[2]; }
};
