#ifndef SRC_FILE_H
#define SRC_FILE_H

#include <filesystem>
#include <string>
#include <fstream>

namespace PB
{
	// throws runtime error
	void WriteFile(std::filesystem::path Path, std::string FileName, std::string Command);

	// throws runtime error
	std::string ReadFile(std::filesystem::path Path, std::string FileName);
}


#endif //SRC_FILE_H