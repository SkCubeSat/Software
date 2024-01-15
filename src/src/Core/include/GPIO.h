#pragma once

#include "Util/File.h"

#include<sstream>
#include <unistd.h>
#include <gpiod.h>

namespace PB
{
	class GPIO
	{
	public:
		enum class Direction
		{
			eInput,
			eOutput
		};
		enum class Value
		{
			eHigh,
			eLow
		};

		explicit GPIO(uint32_t Number);
		~GPIO();

		void SetDirection(Direction Direction);
		void SetValue(Value Value);

		// General read write, use at your own risk
		void Write(std::string FileName, std::string Value);
		std::string Read(std::string FileName);

		Direction GetDirection();
		Value GetValue();

	private:
		uint32_t m_Number;
		std::filesystem::path m_FilePath;
	};
}