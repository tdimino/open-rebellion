
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005b0330(int param_1,float param_2)

{
  bool bVar1;
  float fVar2;
  
  if ((_DAT_0066c484 < *(float *)(param_1 + 0x3b4)) &&
     (fVar2 = *(float *)(param_1 + 0x3b4) - param_2 * _DAT_0066c48c, bVar1 = fVar2 < _DAT_0066c484,
     *(float *)(param_1 + 0x3b4) = fVar2, bVar1)) {
    *(undefined4 *)(param_1 + 0x3b4) = 0;
  }
  if ((_DAT_0066c484 < *(float *)(param_1 + 0x3b0)) &&
     (fVar2 = *(float *)(param_1 + 0x3b0) - param_2 * _DAT_0066c48c, bVar1 = fVar2 < _DAT_0066c484,
     *(float *)(param_1 + 0x3b0) = fVar2, bVar1)) {
    *(undefined4 *)(param_1 + 0x3b0) = 0;
  }
  fVar2 = _DAT_0066c484;
  if (((*(int *)(param_1 + 0x124) == 0) && (*(int *)(param_1 + 0x158) != 0)) &&
     (param_2 = *(float *)(param_1 + 0x3f4) - param_2, *(float *)(param_1 + 0x3f4) = param_2,
     param_2 <= fVar2)) {
    *(float *)(param_1 + 0x3f4) = _DAT_0066c46c * _DAT_0066c490;
    FUN_005b1490(param_1);
  }
  return;
}

