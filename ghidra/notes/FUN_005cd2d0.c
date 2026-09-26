
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __fastcall FUN_005cd2d0(int param_1)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float local_28;
  float local_24 [2];
  float local_1c;
  float local_18;
  float local_14;
  float local_4;
  
  fVar1 = *(float *)(param_1 + 0x10);
  local_14 = *(float *)(param_1 + 0x1c);
  local_1c = *(float *)(param_1 + 0x14);
  local_18 = *(float *)(param_1 + 0x18);
  fVar2 = *(float *)(param_1 + 0x20);
  fVar3 = *(float *)(param_1 + 0x24);
  fVar4 = *(float *)(param_1 + 0x28);
  local_4 = *(float *)(param_1 + 0x2c);
  if (local_14 != _DAT_0066d020) {
    if (local_14 == _DAT_0066d024) {
      local_14 = SQRT(fVar1 * fVar1 + local_18 * local_18 + local_1c * local_1c);
    }
    if (_DAT_0066d028 < local_14) {
      fVar5 = _DAT_0066d020 / local_14;
      fVar1 = fVar5 * fVar1;
      local_1c = fVar5 * local_1c;
      local_18 = fVar5 * local_18;
    }
  }
  if (local_4 != _DAT_0066d020) {
    if (local_4 == _DAT_0066d024) {
      local_4 = SQRT(fVar2 * fVar2 + fVar4 * fVar4 + fVar3 * fVar3);
    }
    if (_DAT_0066d028 < local_4) {
      fVar5 = _DAT_0066d020 / local_4;
      fVar2 = fVar5 * fVar2;
      fVar3 = fVar5 * fVar3;
      fVar4 = fVar5 * fVar4;
    }
  }
  local_28 = fVar3 * local_1c + fVar4 * local_18 + fVar2 * fVar1;
  local_24[0] = _DAT_0066d020;
  FUN_005cbdd0(&local_28,local_24);
  local_24[0] = -1.0;
  FUN_005cbdb0(&local_28,local_24);
  FUN_0061a340();
  return;
}

