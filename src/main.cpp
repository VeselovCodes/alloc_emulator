#include"./include/MemoryRepresentation/bitmap_representaation.hpp"

int main() {
	auto bitmap_mem = BitmapMemRepresentation(10);
	bitmap_mem.print();
	bitmap_mem.alloc(2, 12);
	std::cout << bitmap_mem.isFree(2, 4) << std::endl;
	bitmap_mem.print();
	return 0;
}
