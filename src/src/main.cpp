#include <Core.h>
#include <iostream>

#include <gpiod.hpp>

int main()
{
	gpiod::chip Chip{ "gpiochip0" };

	auto Request = Chip.prepare_request();

	Request.add_line_settings()

	return 0;
}