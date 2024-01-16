#include "File.h"

namespace PB
{
	void WriteFile(std::filesystem::path Path, std::string FileName, std::string Command)
	{
		std::ofstream File(Path / FileName);

		if(!File.is_open())
		{ throw std::runtime_error("Cannot open path : " + (Path / FileName).generic_string()); }

		File << Command;
		File.close();
	}

	std::string ReadFile(std::filesystem::path Path, std::string FileName)
	{
		std::ifstream File(Path/FileName);

		if (!File.is_open())
		{ throw std::runtime_error("Failed to open : " + (Path/FileName).generic_string()); }

		std::string FileContent { std::istreambuf_iterator<char>(File), std::istreambuf_iterator<char>() };

		File.close();
		return FileContent;
	}

}
