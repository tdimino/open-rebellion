
int FUN_0054bf00(int param_1,undefined4 *param_2,void *param_3)

{
  void *this;
  bool bVar1;
  void *this_00;
  uint *puVar2;
  undefined3 extraout_var;
  int iVar3;
  int iVar4;
  void *local_24;
  int *local_20;
  int local_1c;
  undefined4 local_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this = param_3;
  puStack_8 = &LAB_00648000;
  pvStack_c = ExceptionList;
  iVar4 = 1;
  local_1c = 1;
  local_18 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_3,&local_1c);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_004ece80(param_2);
  FUN_004ece30(&local_24);
  local_4._0_1_ = 1;
  local_4._1_3_ = 0;
  local_1c = 1;
  local_20 = (int *)0x0;
  for (this_00 = (void *)thunk_FUN_005f5060(param_1); this_00 != (void *)0x0;
      this_00 = *(void **)((int)this_00 + 0x10)) {
    if (*(int *)((int)this + 4) != -1) goto LAB_0054c0be;
    local_20 = (int *)0x0;
    puVar2 = FUN_00403040(this_00,&param_1);
    local_4._0_1_ = 2;
    bVar1 = FUN_00582c30(puVar2,&local_20);
    if ((CONCAT31(extraout_var,bVar1) == 0) || (iVar4 == 0)) {
      iVar4 = 0;
    }
    else {
      iVar4 = 1;
    }
    local_4._0_1_ = 1;
    FUN_00619730();
    if (local_20 != (int *)0x0) {
      FUN_004ece30(&param_3);
      local_4._0_1_ = 3;
      iVar3 = (**(code **)(*local_20 + 0xc))(&param_3);
      if ((iVar3 == 0) || (iVar4 == 0)) {
        iVar4 = 0;
      }
      else {
        iVar4 = 1;
      }
      puVar2 = FUN_004ece40((uint *)&param_3);
      if (puVar2 != (uint *)0x0) {
        if (local_1c == 0) {
          if (local_24 != param_3) {
            uStack_14 = 0x40;
            uStack_10 = 3;
            local_4._0_1_ = 4;
            FUN_00520580(this,&uStack_14);
            local_4._0_1_ = 3;
            FUN_00619730();
          }
        }
        else {
          local_1c = 0;
          FUN_004f26d0(&local_24,&param_3);
        }
      }
      local_4._0_1_ = 1;
      FUN_00619730();
    }
  }
  if ((*(int *)((int)this + 4) == -1) && (local_1c != 0)) {
    uStack_14 = 1;
    uStack_10 = 0x16;
    local_4._0_1_ = 5;
    FUN_00520580(this,&uStack_14);
    local_4._0_1_ = 1;
    FUN_00619730();
  }
LAB_0054c0be:
  puVar2 = FUN_004ece40((uint *)&local_24);
  if (puVar2 != (uint *)0x0) {
    FUN_004f26d0(param_2,&local_24);
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return iVar4;
}

