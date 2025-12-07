// C言語として関数を実装する
extern "C"
{
    #include "wmx.h"
}

#include "C:\\Program Files\\SoftServo\\WMX3\\Include\\WMX3Api.h"
#include "C:\\Program Files\\SoftServo\\WMX3\\Include\\CoreMotionApi.h"

using namespace wmx3Api;

WMX3Api dev;
CoreMotion coreMotion(&dev);
CoreMotionStatus cmStatus;

int open_wmx()
{
    int ret = dev.CreateDevice("C:\\Program Files\\SoftServo\\WMX3");
    if (ret != 0){
        return ret;
    }

    return 0;
}

int close_wmx()
{
    int ret = dev.CloseDevice();

    return ret;
}

double get_pos(int axis)
{
    int ret = coreMotion.GetStatus(&cmStatus);
    if (ret != 0)
    {
        return 0;
    }

    return cmStatus.axesStatus[axis].posCmd;
}
