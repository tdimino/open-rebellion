
bool FUN_00548910(int *param_1,int *param_2,void *param_3)

{
  bool bVar1;
  short sVar2;
  int iVar3;
  uint uVar4;
  uint *puVar5;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  void *this;
  void *pvVar6;
  undefined3 extraout_var_02;
  bool bVar7;
  uint local_8c;
  void *local_88;
  uint local_84;
  int local_80;
  int local_7c;
  int local_78;
  uint *local_74;
  int local_70;
  uint local_6c;
  uint local_68;
  uint local_64;
  uint local_60;
  uint local_5c;
  int local_58;
  undefined4 local_54 [2];
  undefined4 local_4c [2];
  undefined4 local_44 [2];
  undefined4 local_3c [11];
  int *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647b4e;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_2 = 0;
  FUN_005834d0(local_44);
  local_4 = 0;
  iVar3 = FUN_00520ec0(param_1,(int)local_44);
  bVar7 = iVar3 != 0;
  local_5c = (uint)param_1[9] >> 6 & 3;
  local_58 = FUN_00520dd0(param_1);
  local_70 = 0;
  FUN_004ece30(&local_8c);
  local_4._0_1_ = 1;
  FUN_004ece30(&local_84);
  local_4._0_1_ = 2;
  FUN_004f4340(local_54);
  local_4._0_1_ = 3;
  local_80 = 0;
  FUN_00525bb0(local_3c,param_1);
  local_4._0_1_ = 4;
  FUN_00525930((int)local_3c);
  while (local_10 != (int *)0x0) {
    local_7c = 0;
    local_78 = 0;
    local_74 = (uint *)0x0;
    iVar3 = FUN_00521220(param_1,local_10,&local_7c,&local_78,&local_74);
    if ((iVar3 == 0) || (!bVar7)) {
      bVar7 = false;
    }
    else {
      bVar7 = true;
    }
    if (((local_7c != 0) && (local_78 != 0)) && ((*(byte *)(local_10 + 0x1e) & 4) == 0)) {
      uVar4 = FUN_00542ee0(local_10,param_1,local_74,&local_84);
      if ((uVar4 == 0) || (!bVar7)) {
        bVar7 = false;
      }
      else {
        bVar7 = true;
      }
      if (local_70 == 0) {
        local_70 = 1;
        FUN_004f26d0(&local_8c,&local_84);
      }
      if (local_8c == local_84) {
        puVar5 = FUN_004025b0(local_10,&local_6c);
        local_4._0_1_ = 5;
        FUN_004f44b0(local_54,puVar5,0);
        local_4._0_1_ = 4;
        FUN_00619730();
        puVar5 = FUN_004025b0(local_10,&local_64);
        local_4._0_1_ = 6;
        bVar1 = FUN_00520bd0(param_1,puVar5);
        if (CONCAT31(extraout_var,bVar1) == 0) {
          puVar5 = FUN_004025b0(local_10,&local_68);
          local_4._0_1_ = 7;
          bVar1 = FUN_00520bf0(param_1,puVar5);
          local_4._0_1_ = 6;
          FUN_00619730();
          if (CONCAT31(extraout_var_00,bVar1) != 0) goto LAB_00548b27;
          bVar1 = false;
        }
        else {
LAB_00548b27:
          bVar1 = true;
        }
        local_4._0_1_ = 4;
        FUN_00619730();
        if (bVar1) {
          local_80 = 1;
          iVar3 = FUN_004ece60(&local_8c);
          if ((iVar3 != 0) &&
             (bVar1 = FUN_00520e40((int)param_1), CONCAT31(extraout_var_01,bVar1) != 0)) {
            FUN_004f4390(local_4c,(int)(param_1 + 0x27));
            local_4._0_1_ = 8;
            puVar5 = FUN_004025b0(local_10,&local_60);
            local_4._0_1_ = 9;
            FUN_004f44b0(local_4c,puVar5,0);
            local_4._0_1_ = 8;
            FUN_00619730();
            FUN_00522070(param_1,local_4c,param_3);
            local_4._0_1_ = 4;
            FUN_004f4380(local_4c);
          }
        }
      }
    }
    FUN_005258f0((int)local_3c);
  }
  sVar2 = FUN_005f50e0((int)local_54);
  if (sVar2 != 0) {
    *param_2 = 1;
    for (this = (void *)thunk_FUN_005f5060((int)local_54); this != (void *)0x0;
        this = *(void **)((int)this + 0x10)) {
      pvVar6 = FUN_00403040(this,&local_60);
      local_4._0_1_ = 10;
      iVar3 = (**(code **)(*param_1 + 0x1d0))(pvVar6,local_80,param_3);
      if ((iVar3 == 0) || (!bVar7)) {
        bVar7 = false;
      }
      else {
        bVar7 = true;
      }
      local_4._0_1_ = 4;
      FUN_00619730();
    }
    if (local_80 == 0) {
      iVar3 = FUN_00549850((int)local_54,param_3);
      if ((iVar3 != 0) && (bVar7)) {
        bVar7 = true;
        goto LAB_00548cfa;
      }
    }
    else {
      iVar3 = FUN_004ece60(&local_8c);
      if (iVar3 == 0) {
        iVar3 = FUN_00549580((uint)local_54,param_3);
      }
      else {
        bVar1 = FUN_005475f0(local_88,local_5c,local_44,&local_8c,(int)local_54,
                             (uint)param_1[0x29] >> 2 & 1,(uint)param_3);
        iVar3 = CONCAT31(extraout_var_02,bVar1);
      }
      if ((iVar3 != 0) && (bVar7)) {
        bVar7 = true;
        goto LAB_00548cfa;
      }
    }
    bVar7 = false;
  }
LAB_00548cfa:
  bVar1 = false;
  if (bVar7) {
    iVar3 = FUN_00520dd0(param_1);
    if ((iVar3 == local_58) && (*param_2 != 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
  }
  local_4._0_1_ = 3;
  FUN_00525c50(local_3c);
  local_4._0_1_ = 2;
  FUN_004f4380(local_54);
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00583500(local_44);
  ExceptionList = pvStack_c;
  return bVar1;
}

