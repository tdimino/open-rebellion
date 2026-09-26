
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float10 __fastcall FUN_005fbbe0(int param_1)

{
  DWORD DVar1;
  uint uVar2;
  float10 fVar3;
  LARGE_INTEGER local_8;
  
  if (*(float *)(param_1 + 8) == _DAT_0066dea4) {
    DVar1 = timeGetTime();
    fVar3 = (float10)(DVar1 - *(int *)(param_1 + 0x10)) * (float10)_DAT_0066deac;
  }
  else {
    QueryPerformanceCounter(&local_8);
    uVar2 = *(uint *)(param_1 + 0x10);
    if (local_8.s.LowPart < uVar2) {
      local_8.s.HighPart = local_8.s.HighPart + -1;
      uVar2 = local_8.s.LowPart + 1 + ~uVar2;
    }
    else {
      uVar2 = local_8.s.LowPart - uVar2;
    }
    fVar3 = (float10)uVar2 * (float10)*(float *)(param_1 + 8);
    if (local_8.s.HighPart != *(int *)(param_1 + 0x14)) {
      return (float10)*(float *)(param_1 + 8) * (float10)_DAT_0066dea8 *
             (float10)(local_8.s.HighPart - *(int *)(param_1 + 0x14)) + fVar3;
    }
  }
  return fVar3;
}

