
int __cdecl FUN_00570bc0(uint param_1,int param_2,void *param_3,undefined4 *param_4)

{
  int iVar1;
  int iVar2;
  void *this;
  undefined4 *puVar3;
  bool bVar4;
  undefined3 extraout_var;
  uint *puVar5;
  int *piVar6;
  int *piVar7;
  int local_34;
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
  puStack_8 = &LAB_0064c180;
  local_c = ExceptionList;
  local_34 = 1;
  local_2c = 1;
  local_28 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_2c);
  local_4 = 0xffffffff;
  FUN_00619730();
  puVar3 = param_4;
  param_3 = (void *)0x0;
  *param_4 = 0;
  if (*(int *)((int)this + 4) == -1) {
    bVar4 = FUN_00570f80(*(int *)(param_2 + 0x10),*(int *)(param_2 + 0x18),(int *)&param_3);
    local_34 = CONCAT31(extraout_var,bVar4);
    if ((local_34 == 0) || (param_3 == (void *)0x0)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    if ((local_34 != 0) && (!bVar4)) {
      local_2c = 0x40;
      local_28 = 0x90;
      local_4 = 1;
      FUN_00520580(this,&local_2c);
      local_4 = 0xffffffff;
      FUN_00619730();
      *puVar3 = 0xffffffff;
    }
  }
  param_4 = (undefined4 *)thunk_FUN_005f5060(param_1);
  while (((param_4 != (undefined4 *)0x0 && (local_34 != 0)) && (*(int *)((int)this + 4) == -1))) {
    puVar5 = FUN_00559950(param_4,&param_2);
    local_4 = 2;
    local_34 = FUN_00582e90(puVar5,&local_20,&local_24,&local_30);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (((local_34 != 0) && (local_30 != (void *)0x0)) && (param_3 != (void *)0x0)) {
      if (*(int *)((int)local_30 + 0xa0) + *(int *)((int)local_30 + 0xa4) == 0) {
        local_1c = 0x40;
        local_18 = 5;
        local_4 = 3;
        FUN_00520580(this,&local_1c);
        local_4 = 0xffffffff;
        FUN_00619730();
        *puVar3 = 0xffffffff;
      }
      if (*(int *)((int)this + 4) == -1) {
        piVar6 = (int *)FUN_00402e40(param_3,&local_2c);
        local_4 = 4;
        piVar7 = (int *)FUN_00402e40(local_30,&param_1);
        iVar1 = *piVar7;
        iVar2 = *piVar6;
        FUN_00619730();
        local_4 = 0xffffffff;
        FUN_00619730();
        if ((iVar1 != iVar2) &&
           (*(int *)((int)param_3 + 0xa0) + *(int *)((int)param_3 + 0xa4) <=
            *(int *)((int)local_30 + 0xa0))) {
          local_14 = 0x40;
          local_10 = 0xd;
          local_4 = 5;
          FUN_00520580(this,&local_14);
          local_4 = 0xffffffff;
          FUN_00619730();
          *puVar3 = 0xffffffff;
        }
      }
    }
    param_4 = (undefined4 *)param_4[4];
  }
  ExceptionList = local_c;
  return local_34;
}

