
bool __thiscall FUN_00560d50(void *this,uint *param_1,uint *param_2,uint *param_3,void *param_4)

{
  bool bVar1;
  bool bVar2;
  bool bVar3;
  int *piVar4;
  uint uVar5;
  undefined3 extraout_var;
  void *pvVar6;
  void *pvVar7;
  int iVar8;
  uint *puVar9;
  void *this_00;
  undefined4 auStack_2c [7];
  int iStack_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puVar9 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064a228;
  local_c = ExceptionList;
  bVar1 = true;
  if ((*param_1 >> 0x18 < 0x30) || (0x3b < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  ExceptionList = &local_c;
  FUN_00619730();
  if (!bVar1) {
    if ((*puVar9 >> 0x18 < 0x14) || (0x1b < *puVar9 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (!bVar1) {
      ExceptionList = local_c;
      return true;
    }
    piVar4 = (int *)FUN_00504f70(puVar9);
    if (piVar4 == (int *)0x0) {
      ExceptionList = local_c;
      return piVar4 != (int *)0x0;
    }
    local_4 = 6;
    uVar5 = (**(code **)(*piVar4 + 4))();
    if (((uVar5 < 0x14) || (0x1b < uVar5)) || (bVar1 = true, piVar4 == (int *)0x0)) {
      bVar1 = false;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    local_4 = 7;
    uVar5 = (**(code **)(*piVar4 + 4))();
    if ((uVar5 < 0x14) || (0x1b < uVar5)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (!bVar3) {
      ExceptionList = local_c;
      return bVar1;
    }
    FUN_004f25c0(auStack_2c,piVar4,3);
    local_4 = 8;
    FUN_00513120((int)auStack_2c);
    pvVar6 = param_4;
    while (iStack_10 != 0) {
      iVar8 = FUN_0052bed0((int)auStack_2c);
      if (*(short *)(iVar8 + 0x96) != 0) {
        iVar8 = 0;
        pvVar7 = pvVar6;
        this_00 = (void *)FUN_0052bed0((int)auStack_2c);
        iVar8 = FUN_004ee350(this_00,iVar8,pvVar7);
        if ((iVar8 == 0) || (!bVar1)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
      }
      FUN_005130d0((int)auStack_2c);
    }
    local_4 = 0xffffffff;
    FUN_004f26c0(auStack_2c);
    ExceptionList = local_c;
    return bVar1;
  }
  piVar4 = (int *)FUN_00505190(puVar9);
  if (piVar4 == (int *)0x0) {
    ExceptionList = local_c;
    return piVar4 != (int *)0x0;
  }
  local_4 = 0;
  uVar5 = (**(code **)(*piVar4 + 4))();
  if (((uVar5 < 0x30) || (0x3b < uVar5)) || (bVar1 = true, piVar4 == (int *)0x0)) {
    bVar1 = false;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  local_4 = 1;
  uVar5 = (**(code **)(*piVar4 + 4))();
  if ((uVar5 < 0x30) || (0x3b < uVar5)) {
    bVar3 = false;
  }
  else {
    bVar3 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  if (!bVar3) {
    ExceptionList = local_c;
    return bVar1;
  }
  local_4 = 2;
  uVar5 = (**(code **)(*piVar4 + 4))();
  if ((uVar5 < 0x34) || (0x34 < uVar5)) {
    bVar3 = false;
  }
  else {
    bVar3 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  bVar2 = bVar1;
  if ((bVar3) &&
     ((bVar3 = FUN_005617b0(this,param_4), CONCAT31(extraout_var,bVar3) == 0 ||
      (bVar2 = true, !bVar1)))) {
    bVar2 = false;
  }
  puVar9 = param_2;
  if (*(short *)((int)piVar4 + 0x96) == 0) {
    ExceptionList = local_c;
    return bVar2;
  }
  bVar1 = true;
  bVar3 = true;
  local_4 = 3;
  if ((0x13 < *param_2 >> 0x18) && (*param_2 >> 0x18 < 0x1c)) {
    if ((*param_3 >> 0x18 < 0x14) || (0x1b < *param_3 >> 0x18)) {
      bVar1 = false;
    }
    FUN_00619730();
    if (bVar1) {
      bVar1 = true;
      goto LAB_00560f3e;
    }
  }
  bVar1 = false;
LAB_00560f3e:
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    pvVar6 = (void *)FUN_00504dc0(puVar9);
    if ((pvVar6 == (void *)0x0) || (!bVar2)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    pvVar7 = (void *)FUN_00504dc0(param_3);
    if ((pvVar7 == (void *)0x0) || (!bVar1)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_004ece30(&param_2);
    local_4 = 4;
    FUN_004ece30(&param_1);
    local_4 = CONCAT31(local_4._1_3_,5);
    if (pvVar6 != (void *)0x0) {
      iVar8 = FUN_004f6b70(pvVar6,(uint *)&param_2);
      if ((iVar8 == 0) || (!bVar2)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
    }
    if ((pvVar7 != (void *)0x0) &&
       ((iVar8 = FUN_004f6b70(pvVar7,(uint *)&param_1), iVar8 == 0 ||
        (bVar1 = !bVar2, bVar2 = true, bVar1)))) {
      bVar2 = false;
    }
    puVar9 = FUN_004ece40((uint *)&param_2);
    if ((puVar9 != (uint *)0x0) && (puVar9 = FUN_004ece40((uint *)&param_1), puVar9 != (uint *)0x0))
    {
      if (((uint)param_2 >> 0x18 < 8) || (0xf < (uint)param_2 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        if (((uint)param_1 >> 0x18 < 8) || (0xf < (uint)param_1 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if ((bVar1) && (param_2 == param_1)) {
          bVar3 = false;
        }
      }
    }
    local_4 = CONCAT31(local_4._1_3_,4);
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  if (bVar3) {
    iVar8 = FUN_004ee350(piVar4,0,param_4);
    if ((iVar8 == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
  }
  ExceptionList = local_c;
  return bVar2;
}

