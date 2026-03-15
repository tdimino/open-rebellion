
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __fastcall FUN_005d41a0(int *param_1)

{
  int *piVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  undefined4 uVar5;
  float10 fVar6;
  
  uVar5 = 0;
  if ((param_1[0x52] != 0) &&
     (iVar4 = param_1[6], iVar3 = FUN_005c2590(DAT_006bcbd4), iVar3 == iVar4)) {
    iVar4 = (**(code **)(*(int *)param_1[0x4a] + 0xa4))((int *)param_1[0x4a],param_1[0x52],0);
    FUN_005d8b00(iVar4);
    param_1[0x48] = 0;
    param_1[0x52] = 0;
    param_1[0x50] = 0;
    param_1[0x4f] = 0;
    return 1;
  }
  if (param_1[0x50] < param_1[0x4f]) {
    if ((param_1[0x50] == 0) ||
       (fVar6 = FUN_005fbbe0((int)(param_1 + 0x3c)), (float10)_DAT_0066d154 < fVar6)) {
      FUN_005d2f50((int)param_1);
      if (param_1[0x52] != 0) {
        iVar4 = (**(code **)(*(int *)param_1[0x4a] + 0xa4))((int *)param_1[0x4a],param_1[0x52],0);
        FUN_005d8b00(iVar4);
        iVar3 = param_1[0x51];
        iVar2 = param_1[0x50];
        piVar1 = (int *)(DAT_006bcbd4 + 0x168);
        for (iVar4 = FUN_005f5060(DAT_006bcbd4 + 0x16c); iVar4 != 0; iVar4 = *(int *)(iVar4 + 0x10))
        {
          if (*(short *)(iVar4 + 0x18) == (short)((short)iVar3 + (short)iVar2)) goto LAB_005d42b6;
        }
        iVar4 = 0;
LAB_005d42b6:
        if (iVar4 == 0) {
          iVar4 = (**(code **)(*piVar1 + 4))(&stack0xfffffff4);
          param_1[0x52] = iVar4;
        }
        else {
          param_1[0x52] = *(int *)(iVar4 + 0x1c);
        }
        iVar4 = (**(code **)(*(int *)param_1[0x4a] + 0x48))((int *)param_1[0x4a],param_1[0x52],0);
        FUN_005d8b00(iVar4);
      }
      FUN_005fbbb0((int)(param_1 + 0x3c));
    }
  }
  else {
    param_1[0x48] = param_1[0x48] & 0xffffff03;
    if (param_1[0x52] != 0) {
      iVar4 = (**(code **)(*(int *)param_1[0x4a] + 0xa4))((int *)param_1[0x4a],param_1[0x52],0);
      FUN_005d8b00(iVar4);
    }
    param_1[0x52] = 0;
    param_1[0x50] = -1;
    param_1[0x4f] = 0;
    uVar5 = 1;
  }
  if (((*(byte *)(param_1 + 0x48) & 0x80) != 0) && (param_1[0x50] == 0xb)) {
    (**(code **)(*param_1 + 0x24))(1,0);
    (**(code **)(*param_1 + 0x24))(2,0);
    (**(code **)(*param_1 + 4))();
  }
  param_1[0x50] = param_1[0x50] + 1;
  return uVar5;
}

