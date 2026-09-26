
uint __thiscall FUN_0053c4b0(void *this,undefined4 *param_1)

{
  undefined4 *this_00;
  bool bVar1;
  short sVar2;
  int iVar3;
  undefined3 extraout_var;
  uint uVar4;
  undefined4 *puVar5;
  undefined4 *puVar6;
  undefined4 local_2c [2];
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c [4];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_1;
  puStack_8 = &LAB_00646470;
  pvStack_c = ExceptionList;
  local_24 = 1;
  local_20 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_1,&local_24);
  local_4 = 0xffffffff;
  FUN_00619730();
  if ((*(int *)((int)this + 0x44) == 0) || (*(int *)((int)this + 0x48) == 0)) {
    uVar4 = 0;
  }
  else {
    uVar4 = 1;
  }
  if (uVar4 == 0) {
    local_24 = 1;
    local_20 = 1;
    local_4 = 1;
    FUN_00520580(this_00,&local_24);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  else if (this_00[1] == -1) {
    (**(code **)(**(int **)((int)this + 0x44) + 4))();
  }
  FUN_004f4340(local_2c);
  local_4 = 2;
  if (uVar4 != 0) {
    if (this_00[1] == -1) {
      FUN_00541e70(local_1c);
      local_4._0_1_ = 3;
      FUN_0051fcb0(this,local_1c);
      puVar5 = local_2c;
      puVar6 = this_00;
      iVar3 = (**(code **)(*(int *)this + 0x28))();
      uVar4 = FUN_0053f150(local_1c,iVar3,puVar5,puVar6);
      if (((uVar4 != 0) && (this_00[1] == -1)) && (sVar2 = FUN_005f50e0((int)local_2c), sVar2 == 0))
      {
        local_24 = 1;
        local_20 = 0x16;
        local_4._0_1_ = 4;
        FUN_00520580(this_00,&local_24);
        local_4._0_1_ = 3;
        FUN_00619730();
      }
      local_4 = CONCAT31(local_4._1_3_,2);
      FUN_00541ea0();
    }
    if (uVar4 != 0) {
      if (this_00[1] == -1) {
        param_1 = (undefined4 *)0x0;
        do {
          uVar4 = FUN_0053c810(this,local_2c,(int *)&param_1,this_00);
          if (uVar4 == 0) goto LAB_0053c6a8;
          if ((this_00[1] == -1) && (param_1 != (undefined4 *)0x0)) {
            FUN_00536fe0(*(void **)((int)this + 0x44),param_1);
          }
          if (uVar4 == 0) goto LAB_0053c6a8;
        } while ((this_00[1] == -1) && (param_1 != (undefined4 *)0x0));
      }
      if (uVar4 != 0) {
        if (this_00[1] == -1) {
          param_1 = (undefined4 *)0x0;
          bVar1 = FUN_0053ca50(this,(int)local_2c,(int *)&param_1,this_00);
          uVar4 = CONCAT31(extraout_var,bVar1);
          if (uVar4 == 0) goto LAB_0053c6a8;
          if ((this_00[1] == -1) && (param_1 != (undefined4 *)0x0)) {
            FUN_00536fe0(*(void **)((int)this + 0x44),param_1);
          }
        }
        if (uVar4 != 0) goto LAB_0053c6d4;
      }
    }
  }
LAB_0053c6a8:
  local_24 = 1;
  local_20 = 1;
  local_4._0_1_ = 5;
  FUN_00520580(this_00,&local_24);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_00619730();
LAB_0053c6d4:
  if (*(void **)((int)this + 0x48) != (void *)0x0) {
    FUN_00520580(*(void **)((int)this + 0x48),this_00);
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_2c);
  ExceptionList = pvStack_c;
  return uVar4;
}

