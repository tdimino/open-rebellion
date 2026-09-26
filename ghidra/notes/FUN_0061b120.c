
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl FUN_0061b120(int *param_1)

{
  DWORD DVar1;
  int iVar2;
  _SYSTEMTIME _Stack_cc;
  _SYSTEMTIME local_bc;
  _TIME_ZONE_INFORMATION _Stack_ac;
  
  GetLocalTime(&local_bc);
  GetSystemTime(&_Stack_cc);
  if (_Stack_cc.wMinute == DAT_006be77a) {
    if (_Stack_cc.wHour == DAT_006be778) {
      if (_Stack_cc.wDay == DAT_006be776) {
        if (_Stack_cc.wMonth == DAT_006be772) {
          if (_Stack_cc.wYear == DAT_006be770) goto LAB_0061b1ef;
        }
      }
    }
  }
  DVar1 = GetTimeZoneInformation(&_Stack_ac);
  if (DVar1 == 0xffffffff) {
    DAT_006be768 = -1;
  }
  else if (((DVar1 == 2) && (_Stack_ac.DaylightDate.wMonth != 0)) && (_Stack_ac.DaylightBias != 0))
  {
    DAT_006be768 = 1;
  }
  else {
    DAT_006be768 = 0;
  }
  DAT_006be770 = _Stack_cc.wYear;
  DAT_006be772 = _Stack_cc.wMonth;
  _DAT_006be774 = _Stack_cc.wDayOfWeek;
  DAT_006be776 = _Stack_cc.wDay;
  DAT_006be778 = _Stack_cc.wHour;
  DAT_006be77a = _Stack_cc.wMinute;
  _DAT_006be77c = _Stack_cc.wSecond;
  DAT_006be77c_2 = _Stack_cc.wMilliseconds;
LAB_0061b1ef:
  iVar2 = FUN_00624190((uint)local_bc.wYear,(uint)local_bc.wMonth,(uint)local_bc.wDay,
                       (uint)local_bc.wHour,(uint)local_bc.wMinute,(uint)local_bc.wSecond,
                       DAT_006be768);
  if (param_1 != (int *)0x0) {
    *param_1 = iVar2;
  }
  return;
}

