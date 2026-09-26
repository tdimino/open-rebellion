
int __cdecl FUN_00515f40(uint *param_1,int param_2,void *param_3)

{
  bool bVar1;
  void *pvVar2;
  int iVar3;
  undefined3 extraout_var;
  int *piVar4;
  uint uVar5;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  int iVar6;
  int iVar7;
  bool bVar8;
  void *pvVar9;
  void *local_30;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642768;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar2 = (void *)thunk_FUN_00506e40();
  bVar8 = pvVar2 != (void *)0x0;
  if (pvVar2 != (void *)0x0) {
    iVar3 = FUN_00545c30(pvVar2,param_1,param_2,param_3);
    if ((iVar3 == 0) || (!bVar8)) {
      bVar8 = false;
    }
    else {
      bVar8 = true;
    }
  }
  iVar3 = thunk_FUN_00506e80();
  if ((iVar3 == 0) || (!bVar8)) {
    bVar8 = false;
  }
  else {
    bVar8 = true;
  }
  if (iVar3 != 0) {
    bVar1 = FUN_00561280(param_1,param_2,param_3);
    if ((CONCAT31(extraout_var,bVar1) == 0) || (!bVar8)) {
      bVar8 = false;
    }
    else {
      bVar8 = true;
    }
  }
  iVar3 = thunk_FUN_00506e60();
  if ((iVar3 == 0) || (!bVar8)) {
    bVar8 = false;
  }
  else {
    bVar8 = true;
  }
  pvVar2 = (void *)FUN_00505f60(param_1);
  if ((pvVar2 == (void *)0x0) || (!bVar8)) {
    bVar8 = false;
  }
  else {
    bVar8 = true;
  }
  if ((pvVar2 != (void *)0x0) && (param_2 != 0)) {
    iVar3 = FUN_005175e0(pvVar2,param_3);
    if ((iVar3 == 0) || (!bVar8)) {
      bVar8 = false;
    }
    else {
      bVar8 = true;
    }
  }
  if ((pvVar2 != (void *)0x0) && (param_2 != 0)) {
    FUN_004f2c80(local_2c,pvVar2,3);
    local_4 = 0;
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      iVar3 = FUN_0052bed0((int)local_2c);
      if (((*(uint *)(iVar3 + 0x50) >> 2 & 1) == 0) ||
         (iVar3 = FUN_0052bed0((int)local_2c), (*(uint *)(iVar3 + 0x50) >> 4 & 1) != 0)) {
        pvVar9 = param_3;
        piVar4 = (int *)FUN_0052bed0((int)local_2c);
        uVar5 = FUN_00556710(piVar4,pvVar9);
        if ((uVar5 == 0) || (!bVar8)) {
          bVar8 = false;
        }
        else {
          bVar8 = true;
        }
      }
      FUN_005130d0((int)local_2c);
    }
    local_4 = 0xffffffff;
    FUN_004f2d00(local_2c);
  }
  local_30 = (void *)0x0;
  bVar1 = FUN_005187a0(1,(int *)&local_30);
  if ((CONCAT31(extraout_var_00,bVar1) == 0) || (!bVar8)) {
    bVar8 = false;
  }
  else {
    bVar8 = true;
  }
  if (local_30 != (void *)0x0) {
    iVar3 = FUN_0052fff0(local_30,param_3);
    if ((iVar3 == 0) || (!bVar8)) {
      bVar8 = false;
    }
    else {
      bVar8 = true;
    }
  }
  bVar1 = FUN_005187a0(2,(int *)&local_30);
  if ((CONCAT31(extraout_var_01,bVar1) == 0) || (!bVar8)) {
    iVar3 = 0;
  }
  else {
    iVar3 = 1;
  }
  if (local_30 != (void *)0x0) {
    iVar6 = FUN_0052fff0(local_30,param_3);
    if ((iVar6 == 0) || (iVar3 == 0)) {
      iVar3 = 0;
    }
    else {
      iVar3 = 1;
    }
  }
  if ((pvVar2 != (void *)0x0) && (param_2 == 0)) {
    uVar5 = FUN_00509ac0((int)pvVar2);
    if ((uVar5 == 3) || (iVar3 == 0)) {
      iVar3 = 0;
    }
    else {
      iVar3 = 1;
    }
    if (uVar5 != 3) {
      if (((*(byte *)((int)pvVar2 + 0x50) & 8) == 0) &&
         (uVar5 != (*(uint *)((int)pvVar2 + 0x24) >> 6 & 3))) {
        uVar5 = 2 - (uVar5 != 1);
      }
      iVar6 = FUN_00509a20(pvVar2,2);
      iVar7 = FUN_00509a20(pvVar2,1);
      iVar6 = FUN_00517ee0(pvVar2,uVar5,iVar7,iVar6,param_3);
      if ((iVar6 == 0) || (iVar3 == 0)) {
        iVar3 = 0;
      }
      else {
        iVar3 = 1;
      }
    }
  }
  ExceptionList = local_c;
  return iVar3;
}

