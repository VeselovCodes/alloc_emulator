#pragma once

#include<iostream>
#include<vector>
#include<algorithm>

class BitmapMemRepresentation {
public:
	BitmapMemRepresentation() = delete;
	BitmapMemRepresentation(size_t n) : mem_(n, false) {}

	size_t getSize() { return mem_.size(); }
	bool doesRangeFit(size_t offset, size_t size) {
	   	return offset + size < getSize();
   	}
	bool isFree(size_t offset, size_t size) {
	    return std::all_of(mem_.begin() + offset,
						   mem_.begin() + offset + size - 1,
						   [](bool el) { return !el; });
   	}
	void alloc(size_t offset, size_t size) {
		std::fill(mem_.begin() + offset,
			      mem_.begin() + offset + std::min(getSize() - offset, size),
				  true);
	}
	bool salloc(size_t offset,size_t size) {
		if (!doesRangeFit(offset, size) || !isFree(offset, size)) return false;
		alloc(offset, size);
		return true;
	}
	void print() {
		for (auto el : mem_) {
			std::cout << el;
		}
		std::cout << std::endl;
	}
private:
	std::vector<bool> mem_;
};


