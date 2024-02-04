#include <Core.h>
#include <iostream>
#include <thread>

#include <gpiod.hpp>

int main()
{
    const std::filesystem::path ChipPath{"/dev/gpiochip0"};
    const gpiod::line::offset LineOffset{5};

    auto LineValue = gpiod::line::value::ACTIVE;
    auto LineRequest = gpiod::chip{ChipPath}.prepare_request()
            .set_consumer("toggle-line-value").add_line_settings(
                    LineOffset,
                    gpiod::line_settings().set_direction(
                            gpiod::line::direction::OUTPUT))
            .do_request();

    while(true)
    {
        std::cout << LineOffset << "=" << LineValue << ::std::endl;

        std::this_thread::sleep_for(std::chrono::seconds(1));
        LineValue = LineValue == gpiod::line::value::ACTIVE ?
                gpiod::line::value::INACTIVE : gpiod::line::value::ACTIVE;
        request.set_value(LineOffset, LineValue);
    }

	return 0;
}