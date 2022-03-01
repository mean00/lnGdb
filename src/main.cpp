#include "lnArduino.h"

extern "C" void rn_gdb_main();

void setup()
{

}
void loop()
{
    rn_gdb_main();
    lnDelay(10);
}
