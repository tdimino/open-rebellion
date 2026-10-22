
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005d9d80(int param_1,int param_2)

{
  float10 fVar1;
  float10 fVar2;
  
  if (param_2 == 1) {
    fVar2 = (float10)(**(code **)(**(int **)(param_1 + 4) + 0x88))(*(int **)(param_1 + 4));
    fVar1 = (float10)_DAT_0066d470;
    *(float *)(param_1 + 0x38) = (float)(fVar2 * fVar1);
    if (fVar2 * fVar1 < (float10)DAT_0066d438) {
      *(float *)(param_1 + 0x38) = DAT_0066d438;
    }
  }
  else if (param_2 == 2) {
    fVar2 = (float10)(**(code **)(**(int **)(param_1 + 4) + 0x88))(*(int **)(param_1 + 4));
    fVar1 = (float10)_DAT_0066d468;
    *(float *)(param_1 + 0x38) = (float)(fVar2 * fVar1);
    if ((float10)DAT_0066d43c < fVar2 * fVar1) {
      *(float *)(param_1 + 0x38) = DAT_0066d43c;
      return;
    }
  }
  return;
}

