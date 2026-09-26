
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __fastcall FUN_005da830(int param_1)

{
  int iVar1;
  int iVar2;
  float10 fVar3;
  float10 fVar4;
  float local_1c;
  float fStack_18;
  float fStack_14;
  float fStack_4;
  
  iVar1 = param_1 + 0x40;
  if (*(float *)(param_1 + 0x6c) == _DAT_0066d4c4) {
    fVar3 = FUN_005fbbe0(iVar1);
    (**(code **)(**(int **)(param_1 + 0x60) + 0x60))
              (*(int **)(param_1 + 0x60),*(undefined4 *)(param_1 + 0x30),&local_1c);
    fStack_4 = SQRT(local_1c * local_1c + fStack_14 * fStack_14 + fStack_18 * fStack_18);
    fVar4 = FUN_005fbbe0(iVar1);
    fVar3 = (float10)(float)fVar3;
    fVar4 = (fVar4 * (float10)fStack_4) / (float10)_DAT_0066d4bc;
  }
  else {
    fVar3 = FUN_005fbbe0(iVar1);
    if ((float10)_DAT_0066d4bc < fVar3) {
      fVar3 = (float10)_DAT_0066d4bc;
    }
    fVar4 = fVar3 / (float10)*(float *)(param_1 + 0x5c);
  }
  *(float *)(param_1 + 0x6c) = (float)fVar4;
  *(float *)(param_1 + 0x5c) = (float)fVar3;
  iVar2 = (**(code **)(**(int **)(param_1 + 0x2c) + 0x90))
                    (*(int **)(param_1 + 0x2c),*(undefined4 *)(param_1 + 0x60),
                     *(undefined4 *)(param_1 + 0x30),0,0);
  FUN_005d8b00(iVar2);
  iVar2 = (**(code **)(**(int **)(param_1 + 0x38) + 0x34))
                    (*(int **)(param_1 + 0x38),0x3f800000,0x3f800000,*(undefined4 *)(param_1 + 0x6c)
                     ,0);
  FUN_005d8b00(iVar2);
  fVar3 = FUN_005fbbe0(iVar1);
  if ((float10)_DAT_0066d4bc < fVar3) {
    *(undefined4 *)(param_1 + 0x28) = 0;
  }
  return;
}

