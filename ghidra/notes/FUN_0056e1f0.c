
uint __cdecl FUN_0056e1f0(int param_1,undefined4 param_2,void *param_3,undefined4 *param_4)

{
  int iVar1;
  bool bVar2;
  void *this;
  undefined3 uVar3;
  void *this_00;
  int *piVar4;
  uint *puVar5;
  uint local_4c;
  undefined1 local_48 [4];
  undefined4 local_44;
  undefined1 local_40 [4];
  undefined4 local_3c;
  undefined1 local_38 [4];
  undefined1 local_34 [4];
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this = param_3;
  puStack_8 = &LAB_0064bd68;
  local_c = ExceptionList;
  local_4c = 1;
  local_2c = 1;
  local_28 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_2c);
  local_4 = 0xffffffff;
  FUN_00619730();
  *param_4 = 0;
  this_00 = (void *)thunk_FUN_005f5060(param_1);
  do {
    if (((this_00 == (void *)0x0) || (local_4c == 0)) || (*(int *)((int)this + 4) != -1)) {
      ExceptionList = local_c;
      return local_4c;
    }
    piVar4 = FUN_00403040(this_00,local_38);
    local_3c = 0x32000242;
    local_4._1_3_ = 0;
    uVar3 = local_4._1_3_;
    local_4._0_1_ = 2;
    local_4._1_3_ = 0;
    if (*piVar4 == 0x32000242) {
LAB_0056e32d:
      local_4._1_3_ = uVar3;
      bVar2 = false;
    }
    else {
      piVar4 = FUN_00403040(this_00,local_48);
      iVar1 = *piVar4;
      local_4._0_1_ = 3;
      FUN_00619730();
      local_4._0_1_ = 2;
      FUN_00619730();
      uVar3 = local_4._1_3_;
      if (iVar1 == 0x31000241) goto LAB_0056e32d;
      piVar4 = FUN_00403040(this_00,local_40);
      local_44 = 0x38000343;
      iVar1 = *piVar4;
      local_4._0_1_ = 4;
      FUN_00619730();
      local_4._0_1_ = 2;
      FUN_00619730();
      uVar3 = local_4._1_3_;
      if (iVar1 == 0x38000343) goto LAB_0056e32d;
      bVar2 = true;
    }
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar2) {
      local_24 = 0x40;
      local_20 = 10;
      local_4 = 5;
      FUN_00520580(this,&local_24);
      local_4 = 0xffffffff;
      FUN_00619730();
      *param_4 = 0xffffffff;
    }
    if (*(int *)((int)this + 4) == -1) {
      puVar5 = FUN_00403040(this_00,local_34);
      local_4 = 6;
      local_4c = FUN_00582cd0(puVar5,&local_2c,&local_30,&param_3);
      local_4 = 0xffffffff;
      FUN_00619730();
      if (local_4c != 0) {
        local_4c = (uint)(param_3 != (void *)0x0);
        goto LAB_0056e3e3;
      }
    }
    else {
LAB_0056e3e3:
      if (local_4c != 0) {
        if ((*(int *)((int)this + 4) == -1) && ((*(byte *)((int)param_3 + 0xac) & 1) != 0)) {
          local_1c = 0x40;
          local_18 = 9;
          local_4 = 7;
          FUN_00520580(this,&local_1c);
          local_4 = 0xffffffff;
          FUN_00619730();
          *param_4 = 0xffffffff;
        }
        if (((local_4c != 0) && (*(int *)((int)this + 4) == -1)) &&
           ((*(byte *)((int)param_3 + 0x78) & 1) != 0)) {
          local_14 = 0x40;
          local_10 = 8;
          local_4 = 8;
          FUN_00520580(this,&local_14);
          local_4 = 0xffffffff;
          FUN_00619730();
          *param_4 = 0xffffffff;
        }
      }
    }
    this_00 = *(void **)((int)this_00 + 0x10);
  } while( true );
}

