#include <iostream>
#include "test_lib.h"

using namespace std;

int main()
{
  auto graph = load_hashgraph();
  auto handles = graph_handles(graph);
  for (;;) {
    auto x = handles_next(handles);
    if (x == 0) {
        break;
      } else {
      cout << "Handle: " << x << endl;
    }
  }
  return 0;
}
