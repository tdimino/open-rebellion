
undefined4 __cdecl FUN_00601ea0(uint param_1)

{
  MMRESULT MVar1;
  uint uVar2;
  timecaps_tag local_8;
  
  if ((DAT_006be5b8 != 1) && (MVar1 = timeGetDevCaps(&local_8,8), MVar1 == 0)) {
    uVar2 = local_8.wPeriodMin;
    if (local_8.wPeriodMin <= param_1) {
      uVar2 = param_1;
    }
    DAT_006ac7d4 = local_8.wPeriodMax;
    if ((uVar2 < local_8.wPeriodMax) &&
       (DAT_006ac7d4 = local_8.wPeriodMin, local_8.wPeriodMin <= param_1)) {
      DAT_006ac7d4 = param_1;
    }
    if (DAT_006ac7d4 <= param_1) {
      timeBeginPeriod(DAT_006ac7d4);
      DAT_006be5b8 = 1;
      return 1;
    }
  }
  return 0;
}

