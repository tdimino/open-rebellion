
undefined4 __thiscall FUN_00550200(void *this,int *param_1,uint *param_2,int param_3,void *param_4)

{
  bool bVar1;
  uint *puVar2;
  uint uVar3;
  undefined4 *puVar4;
  void *pvVar5;
  int iVar6;
  void *this_00;
  undefined4 uVar7;
  undefined4 local_44;
  undefined4 local_40;
  int iStack_3c;
  undefined4 uStack_38;
  undefined4 uStack_34;
  int iStack_30;
  undefined4 auStack_2c [7];
  int iStack_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006489a8;
  local_c = ExceptionList;
  local_44 = 1;
  local_40 = 0xffffffff;
  iVar6 = 0;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_4,&local_44);
  local_4 = 0xffffffff;
  FUN_00619730();
  bVar1 = false;
  if (*(int *)((int)param_4 + 4) == -1) {
    for (; iVar6 < 9; iVar6 = iVar6 + 1) {
      puVar2 = (uint *)FUN_0054f5b0(iVar6);
      uVar3 = (**(code **)(*param_1 + 4))();
      if ((*puVar2 <= uVar3) && (uVar3 < puVar2[1])) {
        bVar1 = true;
      }
    }
    if (!bVar1) {
      local_40 = 0x19;
      local_44 = 1;
      local_4 = 1;
      FUN_00520580(param_4,&local_44);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  if (*(int *)((int)param_4 + 4) != -1) {
    ExceptionList = local_c;
    return 1;
  }
  if ((param_1[0x14] & 8U) == 0) {
    if ((param_1[0x14] & 2U) != 0) goto LAB_00550339;
    local_44 = 1;
    local_40 = 0x1a;
    local_4 = 3;
    FUN_00520580(param_4,&local_44);
  }
  else {
    local_44 = 1;
    local_40 = 0x12;
    local_4 = 2;
    FUN_00520580(param_4,&local_44);
  }
  local_4 = 0xffffffff;
  FUN_00619730();
LAB_00550339:
  if (*(int *)((int)param_4 + 4) == -1) {
    this_00 = (void *)((int)this + 0x78);
    for (iVar6 = 0; iVar6 < 9; iVar6 = iVar6 + 1) {
      puVar2 = (uint *)FUN_0054f5b0(iVar6);
      uVar3 = (**(code **)(*param_1 + 4))();
      if ((*puVar2 <= uVar3) && (uVar3 < puVar2[1])) {
        FUN_0054fa00(&local_44,(int)this_00 + -0x70);
        local_4 = 4;
        local_40 = 1;
        if (param_3 == 0) {
LAB_005503af:
          uVar3 = FUN_004f6b90(param_1,*param_2);
          if (uVar3 != 0) {
LAB_005503c1:
            iStack_3c = iStack_3c + 1;
          }
        }
        else {
          uVar3 = FUN_004f6b90(param_1,*param_2);
          if (uVar3 == 0) goto LAB_005503c1;
          if (param_3 == 0) goto LAB_005503af;
        }
        FUN_005f3e10((void *)((int)this_00 + -0x70),(int)&local_44);
        local_4 = 0xffffffff;
        FUN_0054f9f0(&local_44);
      }
      FUN_0054fa00(&uStack_38,(int)this_00);
      uVar7 = 4;
      local_4 = 5;
      puVar4 = (undefined4 *)FUN_0054f5b0(iVar6);
      FUN_00513050(auStack_2c,param_1,puVar4,uVar7);
      local_4 = CONCAT31(local_4._1_3_,6);
      FUN_00513120((int)auStack_2c);
      while (iStack_10 != 0) {
        uStack_34 = 1;
        if (param_3 == 0) {
LAB_0055044f:
          uVar3 = *param_2;
          pvVar5 = (void *)FUN_0052bed0((int)auStack_2c);
          uVar3 = FUN_004f6b90(pvVar5,uVar3);
          if (uVar3 != 0) {
LAB_0055046a:
            iStack_30 = iStack_30 + 1;
          }
        }
        else {
          uVar3 = *param_2;
          pvVar5 = (void *)FUN_0052bed0((int)auStack_2c);
          uVar3 = FUN_004f6b90(pvVar5,uVar3);
          if (uVar3 == 0) goto LAB_0055046a;
          if (param_3 == 0) goto LAB_0055044f;
        }
        FUN_005130d0((int)auStack_2c);
      }
      FUN_005f3e10(this_00,(int)&uStack_38);
      local_4 = CONCAT31(local_4._1_3_,5);
      FUN_00513040(auStack_2c);
      local_4 = 0xffffffff;
      FUN_0054f9f0(&uStack_38);
      this_00 = (void *)((int)this_00 + 0xc);
    }
  }
  ExceptionList = local_c;
  return 1;
}

