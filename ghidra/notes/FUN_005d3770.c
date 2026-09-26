
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __fastcall FUN_005d3770(void *param_1)

{
  float fVar1;
  float fVar2;
  int iVar3;
  float10 fVar4;
  float fVar5;
  float fStack_24;
  float fStack_20;
  float fStack_1c;
  undefined1 auStack_c [12];
  
  iVar3 = *(int *)(DAT_006bcbd4 + 0x638);
  fVar5 = *(float *)(iVar3 + 0x14);
  fVar1 = *(float *)(iVar3 + 0x18);
  fVar2 = *(float *)(iVar3 + 0x1c);
  iVar3 = (**(code **)(**(int **)((int)param_1 + 0x2c) + 0x60))
                    (*(int **)((int)param_1 + 0x2c),*(undefined4 *)(iVar3 + 8),auStack_c,0);
  FUN_005d8b00(iVar3);
  fVar5 = fVar2 * fStack_1c + fVar1 * fStack_20 + fVar5 * fStack_24;
  if (fVar5 < _DAT_0066d180) {
    fVar5 = -fVar5;
  }
  fVar4 = (float10)(**(code **)(**(int **)(DAT_006bcbd4 + 0x638) + 0x14))();
  if (*(int *)(DAT_006bcbd4 + 0x688) == 0) {
    iVar3 = 2;
    if ((float10)fVar5 <= (float10)_DAT_0066d15c / fVar4) {
      iVar3 = 1;
    }
  }
  else {
    if ((((float)((float10)_DAT_0066d160 / fVar4) < fVar5) &&
        (iVar3 = 2, *(int *)((int)param_1 + 0x40) != 2)) ||
       (((float)((float10)_DAT_0066d158 / fVar4) < fVar5 &&
        ((fVar5 < (float)((float10)_DAT_0066d160 / fVar4) &&
         (iVar3 = 1, *(int *)((int)param_1 + 0x40) != 1)))))) goto LAB_005d38b9;
    if (((float)((float10)_DAT_0066d158 / fVar4) <= fVar5) || (*(int *)((int)param_1 + 0x40) == 0))
    {
      iVar3 = *(int *)((int)param_1 + 0x40);
    }
    else {
      iVar3 = 0;
    }
  }
  if (iVar3 == *(int *)((int)param_1 + 0x40)) {
    return;
  }
LAB_005d38b9:
  FUN_005d3650(param_1,iVar3);
  return;
}

