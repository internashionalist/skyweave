#pragma once
#include <array>
#include <vector>
#include <string>
#include <chrono>
#include <algorithm>

// for the stretch goal of allowing a user to manually control an individual UAV
enum class UAVControleMode {
	AUTONOMOUS,
	MANUAL
};

class UAV {
private:
	int id;
	int port;
	std::vector<std::string> neighbor_address; /* 172.0.0.1:8001, ...*/
	UAVControleMode mode = UAVControleMode::AUTONOMOUS;

	std::array<double, 3> pos;
	std::array<double, 3> vel;

	struct NeighborInfo {
		int id;
		std::array<double, 3> last_known_pos;
		std::chrono::time_point<std::chrono::steady_clock> last_time;
	};
	std::vector<NeighborInfo> neighbor_status;

public:
	// Constructor
	UAV(int set_id, int set_port, double x, double y, double z) :
		id(set_id), port(set_port), pos{x, y, z}, vel{0, 0, 0} {}

	// Setters
	void set_velocity(double x, double y, double z) { vel = {x, y, z}; }
	void set_velx(double x) { vel[0] = x; }
	void set_vely(double y) { vel[1] = y; }
	void set_velz(double z) { vel[2] = z; }
	void set_neighbor_address(std::vector<std::string> addresses) {neighbor_address = addresses; }

	// Getters
	int get_id() const { return id; }
	int get_port() const { return port; }

	std::array<double, 3> get_pos() const { return pos; }
	double get_x() const { return pos[0]; }
	double get_y() const { return pos[1]; }
	double get_z() const { return pos[2]; }

	std::array<double, 3> get_vel() const { return vel; }
	double get_velx() const { return vel[0]; }
	double get_vely() const { return vel[1]; }
	double get_velz() const { return vel[2]; }

	std::vector<std::string> get_neighbor_address() { return neighbor_address; }
	std::vector<NeighborInfo> get_neighbor_status() { return neighbor_status; }

	// Updaters
	void update_position(double dt) {
		pos[0] += vel[0] * dt;
		pos[1] += vel[1] * dt;
		pos[2] += vel[2] * dt; 
	};
	void add_neighbor_address(const std::string& address) { neighbor_address.push_back(address); }
	void remove_neighbor_address(const std::string& address) {
		auto addr_id = std::find(neighbor_address.begin(), neighbor_address.end(), address);
		if (addr_id != neighbor_address.end()) { neighbor_address.erase(addr_id); }
	};
	void update_neighbor_status(int neighbor_id, const std::array<double, 3>& pos) {
		auto now = std::chrono::steady_clock::now();

		for (auto& neighbor : neighbor_status) {
			if (neighbor.id == neighbor_id) {
				neighbor.last_known_pos = pos;
				neighbor.last_time = now;
				return;
			}
		}
		neighbor_status.push_back({neighbor_id, pos, now});
	}
	void remove_stale_neighbors(std::chrono::milliseconds max_age = std::chrono::milliseconds(1000)) {
		auto now = std::chrono::steady_clock::now();

		neighbor_status.erase(
			std::remove_if(neighbor_status.begin(), neighbor_status.end(),
			[now, max_age](const NeighborInfo& neighbor) {
				return (now - neighbor.last_time) > max_age;
			}),
			neighbor_status.end()
		);
	}
	std::vector<NeighborInfo>
};
