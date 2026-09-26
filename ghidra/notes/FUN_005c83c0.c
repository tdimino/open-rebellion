
uint FUN_005c83c0(int param_1,uint param_2,uint param_3)

{
  int *piVar1;
  ushort uVar2;
  void *pvVar4;
  int *piVar5;
  int iVar6;
  int iVar7;
  uint uVar8;
  int iVar9;
  uint uVar10;
  int *piVar11;
  uint uVar12;
  int *local_20;
  int *local_1c;
  undefined1 *local_18;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  uint uVar3;
  
  uVar12 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653ebb;
  local_c = ExceptionList;
  piVar11 = (int *)0x0;
  local_1c = (int *)0x0;
  ExceptionList = &local_c;
  FUN_005f5060(param_1);
  uVar2 = FUN_005f50e0(param_1);
  uVar3 = (uint)uVar2;
  iVar9 = 1;
  if (0 < (int)param_3) {
    do {
      if (iVar9 * iVar9 - uVar3 != 0 && (int)uVar3 <= iVar9 * iVar9) break;
      iVar9 = iVar9 + 1;
    } while (iVar9 <= (int)param_3);
  }
  if (uVar3 < 2) {
    uVar10 = 1;
  }
  else {
    uVar10 = iVar9 - 1;
  }
  uVar8 = (param_3 - param_2) + 1;
  if ((int)uVar8 <= (int)uVar10) {
    uVar10 = uVar8;
  }
  local_18 = (undefined1 *)((longlong)(ulonglong)uVar3 / (longlong)(int)uVar10);
  param_2 = 0;
  local_20 = (int *)0x0;
  iVar9 = FUN_005f5060(param_1);
  param_3 = uVar12;
  for (; iVar9 != 0; iVar9 = *(int *)(iVar9 + 0x10)) {
    piVar1 = *(int **)(iVar9 + 0x1c);
    if (((int)param_2 % (int)local_18 == 0) && ((int)local_20 < (int)uVar10)) {
      local_20 = (int *)((int)local_20 + 1);
      switch(param_3) {
      case 0:
        param_1 = 0;
        break;
      case 1:
        param_1 = 0x41000000;
        break;
      case 2:
        param_1 = -0x3f000000;
        break;
      case 3:
        param_1 = 0x41800000;
        break;
      case 4:
        param_1 = -0x3e800000;
        break;
      case 5:
        param_1 = 0x41c00000;
        break;
      case 6:
        param_1 = -0x3e400000;
        break;
      default:
        param_1 = 0x42000000;
      }
      piVar11 = piVar1;
      pvVar4 = (void *)FUN_005aaf90();
      FUN_005a93b0(pvVar4,piVar11);
      piVar11 = (int *)0x0;
      piVar1[0xe] = piVar1[0xe];
      piVar1[0xc] = 0;
      piVar1[0xd] = param_1;
      piVar1[0xf] = -0x40800000;
      piVar5 = piVar1;
      pvVar4 = (void *)FUN_005aaf90();
      FUN_005a9380(pvVar4,piVar5);
      pvVar4 = (void *)FUN_00618b70(0xbc);
      local_4 = 0;
      if (pvVar4 != (void *)0x0) {
        piVar11 = FUN_005cbf50(pvVar4,piVar1[0x3b]);
      }
      local_4 = 0xffffffff;
      if (local_1c == (int *)0x0) {
        local_1c = piVar11;
      }
      piVar5 = (int *)FUN_005aaf90();
      (**(code **)(*piVar5 + 0x4c))();
      (**(code **)(*piVar11 + 8))();
      local_18 = &stack0xffffffb8;
      FUN_005caf70(piVar11,piVar1[0xc],piVar1[0xd],piVar1[0xe],piVar1[0xf]);
      piVar5 = (int *)FUN_005aaf90();
      (**(code **)(*piVar5 + 0x38))();
      piVar1[0x5e] = piVar11[6];
      piVar1[0x5d] = piVar11[6];
      iVar6 = FUN_005aaf90();
      iVar6 = *(int *)(iVar6 + 0x9ec);
      iVar7 = (**(code **)(*local_20 + 4))();
      if (iVar7 == iVar6) {
        iVar6 = piVar11[6];
        uVar12 = param_3;
        pvVar4 = (void *)FUN_0059ed70();
        FUN_005a03f0(pvVar4,uVar12,iVar6);
      }
      param_3 = param_3 + 1;
    }
    else {
      (**(code **)(*piVar11 + 0x4c))();
      (**(code **)(*piVar11 + 0x28))();
    }
    FUN_005cbbc0(piVar11);
    param_2 = param_2 + 1;
  }
  ExceptionList = local_c;
  return ((int)param_2 < 1) - 1 & uVar10;
}

