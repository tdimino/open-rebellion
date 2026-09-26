
bool __thiscall
FUN_00545240(void *this,uint *param_1,undefined4 param_2,int *param_3,int param_4,void *param_5)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  uint uVar3;
  void *pvVar4;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  void *pvVar5;
  undefined3 extraout_var_02;
  undefined3 extraout_var_03;
  undefined3 extraout_var_04;
  int *piVar6;
  undefined3 extraout_var_05;
  bool bVar7;
  uint *puVar8;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647548;
  local_c = ExceptionList;
  bVar7 = true;
  ExceptionList = &local_c;
  bVar1 = FUN_005438a0((int *)param_1);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    iVar2 = FUN_00549910(this,param_5);
    bVar7 = iVar2 != 0;
  }
  bVar1 = false;
  if (param_4 != 0) {
    ExceptionList = local_c;
    return bVar7;
  }
  uVar3 = *param_1 >> 0x18;
  local_4 = 0;
  if ((uVar3 < 0x98) || (0x9f < uVar3)) {
    if ((0x8f < uVar3) && (uVar3 < 0x98)) {
      bVar1 = true;
    }
    FUN_00619730();
    if (!bVar1) {
      if ((*param_1 >> 0x18 < 0x10) || (0x3f < *param_1 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) {
        bVar1 = false;
        goto LAB_00545322;
      }
    }
  }
  bVar1 = true;
LAB_00545322:
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    FUN_00525750(local_2c,this);
    local_4 = 1;
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      puVar8 = param_1;
      pvVar4 = (void *)FUN_0052bed0((int)local_2c);
      bVar1 = FUN_00520c70(pvVar4,(int *)puVar8);
      if (CONCAT31(extraout_var_00,bVar1) == 0) {
        puVar8 = param_1;
        pvVar4 = (void *)FUN_0052bed0((int)local_2c);
        bVar1 = FUN_00520cb0(pvVar4,(int *)puVar8);
        if (CONCAT31(extraout_var_01,bVar1) != 0) goto LAB_0054538f;
      }
      else {
LAB_0054538f:
        pvVar4 = param_5;
        pvVar5 = (void *)FUN_0052bed0((int)local_2c);
        iVar2 = FUN_00522480(pvVar5,pvVar4);
        if ((iVar2 == 0) || (!bVar7)) {
          bVar7 = false;
        }
        else {
          bVar7 = true;
        }
      }
      puVar8 = param_1;
      pvVar4 = (void *)FUN_0052bed0((int)local_2c);
      bVar1 = FUN_00520c70(pvVar4,(int *)puVar8);
      if (CONCAT31(extraout_var_02,bVar1) != 0) {
        iVar2 = FUN_0052bed0((int)local_2c);
        bVar1 = FUN_00520e40(iVar2);
        if (CONCAT31(extraout_var_03,bVar1) == 0) {
          iVar2 = FUN_0052bed0((int)local_2c);
          bVar1 = FUN_00520ae0(iVar2);
          if (CONCAT31(extraout_var_04,bVar1) != 0) {
            piVar6 = (int *)FUN_0052bed0((int)local_2c);
            iVar2 = (**(code **)(*piVar6 + 0x1dc))(7,param_5);
            if ((iVar2 == 0) || (!bVar7)) {
              bVar7 = false;
            }
            else {
              bVar7 = true;
            }
          }
        }
      }
      FUN_005130d0((int)local_2c);
    }
    local_4 = 0xffffffff;
    FUN_005257f0(local_2c);
  }
  if ((*param_1 >> 0x18 < 0x28) || (0x2b < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    FUN_0056d1b0(local_2c,this);
    local_4 = 2;
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      piVar6 = param_3;
      pvVar4 = (void *)FUN_0052bed0((int)local_2c);
      bVar1 = FUN_00520c90(pvVar4,piVar6);
      if (CONCAT31(extraout_var_05,bVar1) != 0) {
        pvVar4 = param_5;
        pvVar5 = (void *)FUN_0052bed0((int)local_2c);
        iVar2 = FUN_00522480(pvVar5,pvVar4);
        if ((iVar2 == 0) || (!bVar7)) {
          bVar7 = false;
        }
        else {
          bVar7 = true;
        }
      }
      FUN_005130d0((int)local_2c);
    }
    local_4 = 0xffffffff;
    FUN_0056d250(local_2c);
  }
  FUN_00619730();
  ExceptionList = local_c;
  return bVar7;
}

