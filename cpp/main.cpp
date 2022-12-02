#include <fstream>
#include <string>
#include <sstream>
#include <iostream>
#include <vector>

std::vector<int> get_calorie_totals(std::ifstream &input_file);

int main()
{
  std::ifstream input_file("../input");
  std::vector<int> totals = get_calorie_totals(input_file);
  std::sort(totals.begin(), totals.end(), std::greater<int>());
  // std::cout << totals << std::endl;
  std::cout << "part 1: " << totals[0] << std::endl;
  std::cout << "part 2: " << totals[0] + totals[1] + totals[2] << std::endl;
  return 0;
}

std::vector<int> get_calorie_totals(std::ifstream &input_file)
{
  std::vector<int> totals;
  std::string line;
  int total = 0;
  while (std::getline(input_file, line))
  {
    if (line == "")
    {
      totals.push_back(total);
      total = 0;
    }
    else
    {
      total += std::stoi(line);
    }
  }
  totals.push_back(total);
  return totals;
}
