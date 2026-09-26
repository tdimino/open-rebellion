
int FUN_00548120(int *param_1,void *param_2)

{
  int iVar1;
  void *this;
  int *this_00;
  void *pvVar2;
  bool bVar3;
  int iVar4;
  uint *puVar5;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  int local_6c;
  int local_68;
  int local_64;
  int local_60;
  undefined4 local_5c;
  uint local_58;
  uint local_54;
  uint local_50;
  uint auStack_4c [2];
  undefined4 auStack_44 [2];
  undefined4 local_3c [11];
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647a60;
  local_c = ExceptionList;
  local_6c = 1;
  ExceptionList = &local_c;
  FUN_00525bb0(local_3c,param_1);
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  FUN_00525930((int)local_3c);
  iVar1 = local_6c;
  do {
    do {
      this = local_10;
      if (this == (void *)0x0) {
        local_4 = 0xffffffff;
        FUN_00525c50(local_3c);
        local_60 = 0;
        local_6c = iVar1;
        do {
          iVar1 = local_6c;
          bVar3 = FUN_00548910(this_00,&local_60,param_2);
          if ((CONCAT31(extraout_var_02,bVar3) == 0) || (local_6c = 1, iVar1 == 0)) {
            local_6c = 0;
          }
        } while (local_60 != 0);
        ExceptionList = local_c;
        return local_6c;
      }
      local_10 = this;
      FUN_005258f0((int)local_3c);
      param_1 = (int *)0x0;
      local_68 = 0;
      local_5c = 0;
      iVar4 = FUN_00521220(this_00,this,&param_1,&local_68,&local_5c);
      if ((iVar4 == 0) || (local_6c = 1, iVar1 == 0)) {
        local_6c = 0;
      }
      iVar1 = local_6c;
    } while (((param_1 == (int *)0x0) || (local_68 != 0)) ||
            ((*(byte *)((int)this + 0x78) & 4) != 0));
    puVar5 = FUN_004025b0(this,&local_54);
    local_4._0_1_ = 1;
    bVar3 = FUN_00520bd0(this_00,puVar5);
    if (CONCAT31(extraout_var,bVar3) == 0) {
      puVar5 = FUN_004025b0(this,&local_58);
      local_4._0_1_ = 2;
      bVar3 = FUN_00520bf0(this_00,puVar5);
      local_4._0_1_ = 1;
      FUN_00619730();
      local_64 = 0;
      if (CONCAT31(extraout_var_00,bVar3) != 0) goto LAB_0054822c;
    }
    else {
LAB_0054822c:
      local_64 = 1;
    }
    local_4._0_1_ = 0;
    FUN_00619730();
    puVar5 = FUN_004025b0(this,&local_50);
    pvVar2 = param_2;
    local_4._0_1_ = 3;
    iVar4 = (**(code **)(*this_00 + 0x1d0))(puVar5,0,param_2);
    if ((iVar4 == 0) || (local_6c = 1, iVar1 == 0)) {
      local_6c = 0;
    }
    local_4._0_1_ = 0;
    FUN_00619730();
    iVar1 = local_6c;
    if ((local_64 != 0) &&
       (bVar3 = FUN_00520e40((int)this_00), CONCAT31(extraout_var_01,bVar3) != 0)) {
      FUN_004f4390(auStack_44,(int)(this_00 + 0x27));
      local_4._0_1_ = 4;
      puVar5 = FUN_004025b0(this,auStack_4c);
      local_4._0_1_ = 5;
      FUN_004f44b0(auStack_44,puVar5,0);
      local_4._0_1_ = 4;
      FUN_00619730();
      FUN_00522070(this_00,auStack_44,pvVar2);
      local_4._0_1_ = 0;
      FUN_004f4380(auStack_44);
    }
  } while( true );
}

