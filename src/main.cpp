#include <iostream>
#include <string>
#include "test_lib.h"

using namespace std;

int main()
{
  string path = "lil.gfa";
  auto graph = load_hashgraph((uint8_t const*)path.c_str(), path.size());

  auto handles = graph_handles(graph);
  while (!handles->finished) {
    auto x = handles_next(handles);
    if (x != 0) {
      cout << "Handle: " << x << endl;
    }
  }

  free_handles_iter(handles);
  free_hashgraph(graph);
  return 0;
}
