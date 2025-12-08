// C言語として関数を実装する
extern "C"
{
    #include "wmx.h"
}

#include "C:\\Program Files\\SoftServo\\WMX3\\Include\\WMX3Api.h"
#include "C:\\Program Files\\SoftServo\\WMX3\\Include\\CoreMotionApi.h"
#include "C:\\Program Files\\SoftServo\\WMX3\\Include\\LogApi.h"

using namespace wmx3Api;

WMX3Api dev;
CoreMotion coreMotion(&dev);
Log log(&dev);
CoreMotionStatus cmStatus;
CoreMotionLogOutput out;

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

int start_memlog(int axis)
{
    CoreMotionLogInput in;
    in.axisSelection.axisCount = 1;
    in.axisSelection.axis[0] = axis;
    in.axisOptions.commandPos = 1;

    int ret = log.OpenMemoryLogBuffer(0);
    if (ret != 0)
    {
        return ret;
    }

    ret = log.SetMemoryLog(0, &in);
    if (ret != 0)
    {
        return ret;
    }

    ret = log.StartMemoryLog(0);
    if (ret != 0)
    {
        return ret;
    }

    return 0;
}

int stop_memlog()
{
    int ret = log.StopMemoryLog(0);
    if (ret != 0)
    {
        return ret;
    }

    Sleep(100);

    ret = log.CloseMemoryLogBuffer(0);
    if (ret != 0)
    {
        return ret;
    }

    return 0;
}

int get_memlog(double pos[1000], long long cycle_counter[1000], size_t* pCount)
{
    int ret;
    int loop_cnt = 1000/100;

    *pCount = 0;

    while (loop_cnt-- > 0)
    {
        ret = log.GetMemoryLogData(0, &out);
        if (ret != 0)
        {
            return ret;
        }

        for (int i = 0; i < out.count; i++)
        {
            pos[*pCount] = out.axisData[i][0].commandPos;
            cycle_counter[*pCount] = out.cycleCounter[i];
            (*pCount)++;
            if (*pCount >= 1000)
            {
                return 0;
            }
        }

        if (out.count != constants::maxMemLogDataSize)
        {
            break;
        }
    }

    return 0;
}
