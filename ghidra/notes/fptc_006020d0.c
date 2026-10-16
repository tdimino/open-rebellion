
/* fptc parameter of timeSetEvent
    */

void fptc_006020d0(undefined4 param_1,undefined4 param_2,int *param_3)

{
  LPCRITICAL_SECTION lpCriticalSection;
  
  if (param_3[1] != 0) {
    param_3[7] = (uint)(param_3[5] == 1);
    return;
  }
  lpCriticalSection = (LPCRITICAL_SECTION)(param_3 + 8);
  EnterCriticalSection(lpCriticalSection);
  if (param_3[1] == 0) {
    param_3[1] = 1;
    (**(code **)(*param_3 + 4))();
    if (0 < param_3[1]) {
      param_3[1] = param_3[1] + -1;
    }
    param_3[7] = 0;
    LeaveCriticalSection(lpCriticalSection);
    return;
  }
  param_3[7] = (uint)(param_3[5] == 1);
  LeaveCriticalSection(lpCriticalSection);
  return;
}

