#include <iostream>
#include <unistd.h>
#include <string>

using namespace std;

int main(int argv, char** args) {
  do {
    cout << args[1] << endl;
    sleep(2);
  } while(true);
}
