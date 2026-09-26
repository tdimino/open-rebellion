
uint __cdecl FUN_00570940(uint param_1,int param_2,void *param_3,undefined4 *param_4)

{
  uint uVar1;
  uint uVar2;
  bool bVar3;
  void *this;
  undefined4 *puVar4;
  uint *puVar5;
  uint *puVar6;
  uint local_34;
  void *local_30;
  uint local_2c;
  undefined4 local_28;
  uint local_24;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this = param_3;
  puStack_8 = &LAB_0064c140;
  local_c = ExceptionList;
  local_34 = 1;
  local_2c = 1;
  local_28 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_2c);
  local_4 = 0xffffffff;
  FUN_00619730();
  puVar4 = param_4;
  param_3 = (void *)0x0;
  *param_4 = 0;
  if (*(int *)((int)this + 4) == -1) {
    local_34 = FUN_00570e40(*(int *)(param_2 + 0x10),*(int *)(param_2 + 0x24),(int *)&param_3);
    if ((local_34 == 0) || (param_3 == (void *)0x0)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    if ((local_34 != 0) && (!bVar3)) {
      local_2c = 0x40;
      local_28 = 0x90;
      local_4 = 1;
      FUN_00520580(this,&local_2c);
      local_4 = 0xffffffff;
      FUN_00619730();
      *puVar4 = 0xffffffff;
    }
  }
  param_4 = (undefined4 *)thunk_FUN_005f5060(param_1);
  while (((param_4 != (undefined4 *)0x0 && (local_34 != 0)) && (*(int *)((int)this + 4) == -1))) {
    puVar5 = FUN_00403040(param_4,&param_2);
    local_4 = 2;
    local_34 = FUN_00582cd0(puVar5,&local_20,&local_24,&local_30);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (((local_34 != 0) && (local_30 != (void *)0x0)) && (param_3 != (void *)0x0)) {
      if (*(short *)((int)local_30 + 0x8c) == 0) {
        local_1c = 0x40;
        local_18 = 5;
        local_4 = 3;
        FUN_00520580(this,&local_1c);
        local_4 = 0xffffffff;
        FUN_00619730();
        *puVar4 = 0xffffffff;
      }
      if (*(int *)((int)this + 4) == -1) {
        puVar5 = FUN_004025b0(param_3,&local_2c);
        local_4 = 4;
        puVar6 = FUN_004025b0(local_30,&param_1);
        uVar1 = *puVar6;
        uVar2 = *puVar5;
        FUN_00619730();
        local_4 = 0xffffffff;
        FUN_00619730();
        if ((uVar1 != uVar2) &&
           (*(short *)((int)param_3 + 0x8c) <= *(short *)((int)local_30 + 0x8c))) {
          local_14 = 0x40;
          local_10 = 0xd;
          local_4 = 5;
          FUN_00520580(this,&local_14);
          local_4 = 0xffffffff;
          FUN_00619730();
          *puVar4 = 0xffffffff;
        }
      }
    }
    param_4 = (undefined4 *)param_4[4];
  }
  ExceptionList = local_c;
  return local_34;
}

