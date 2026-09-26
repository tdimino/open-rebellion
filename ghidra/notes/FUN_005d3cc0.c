
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __fastcall FUN_005d3cc0(int *param_1)

{
  int *piVar1;
  undefined2 extraout_var;
  int iVar2;
  uint uVar3;
  float10 fVar4;
  int *piStack_4;
  
  piStack_4 = param_1;
  fVar4 = FUN_005fbbe0((int)(param_1 + 0x42));
  if ((float10)_DAT_0066d154 < fVar4) {
    if (param_1[0x4e] == 0) {
      (**(code **)(*param_1 + 0x24))(1,0);
      (**(code **)(*param_1 + 0x24))(2,0);
    }
    else {
      (**(code **)(*(int *)param_1[0x49] + 0xa4))((int *)param_1[0x49],param_1[0x4e]);
      param_1[0x4e] = 0;
      piStack_4 = (int *)CONCAT22(extraout_var,(short)param_1[0x4d] + (short)param_1[0x4c]);
      piVar1 = (int *)(DAT_006bcbd4 + 0x168);
      for (iVar2 = FUN_005f5060(DAT_006bcbd4 + 0x16c); iVar2 != 0; iVar2 = *(int *)(iVar2 + 0x10)) {
        if (*(short *)(iVar2 + 0x18) == (short)piStack_4) goto LAB_005d3d49;
      }
      iVar2 = 0;
LAB_005d3d49:
      if (iVar2 == 0) {
        iVar2 = (**(code **)(*piVar1 + 4))(&piStack_4);
        param_1[0x4e] = iVar2;
      }
      else {
        param_1[0x4e] = *(int *)(iVar2 + 0x1c);
      }
      if (param_1[0x4e] != 0) {
        (**(code **)(*(int *)param_1[0x49] + 0x48))((int *)param_1[0x49],param_1[0x4e]);
      }
    }
    FUN_005fbbb0((int)(param_1 + 0x42));
    uVar3 = (int)(param_1[0x4c] + 1U) >> 0x1f;
    param_1[0x4c] = ((param_1[0x4c] + 1U ^ uVar3) - uVar3 & 7 ^ uVar3) - uVar3;
  }
  FUN_005d3060((int)param_1);
  return;
}

