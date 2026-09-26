
undefined4 __thiscall FUN_005504d0(void *this,uint param_1,int param_2,void *param_3)

{
  bool bVar1;
  uint *puVar2;
  int iVar3;
  void *this_00;
  undefined4 local_18;
  undefined4 local_14;
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006489d8;
  local_c = ExceptionList;
  local_18 = 1;
  local_14 = 0xffffffff;
  bVar1 = false;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_18);
  local_4 = 0xffffffff;
  FUN_00619730();
  if (*(int *)((int)param_3 + 4) == -1) {
    for (iVar3 = 0; iVar3 < 9; iVar3 = iVar3 + 1) {
      puVar2 = (uint *)FUN_0054f5b0(iVar3);
      if ((*puVar2 <= param_1) && (param_1 < puVar2[1])) {
        bVar1 = true;
      }
    }
    if (!bVar1) {
      local_14 = 0x19;
      local_18 = 1;
      local_4 = 1;
      FUN_00520580(param_3,&local_18);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  if (*(int *)((int)param_3 + 4) == -1) {
    this_00 = (void *)((int)this + 8);
    for (iVar3 = 0; iVar3 < 9; iVar3 = iVar3 + 1) {
      puVar2 = (uint *)FUN_0054f5b0(iVar3);
      if ((*puVar2 <= param_1) && (param_1 < puVar2[1])) {
        FUN_0054fa00(&local_18,(int)this_00);
        local_10 = local_10 + param_2;
        local_4 = 2;
        local_14 = 1;
        FUN_005f3e10(this_00,(int)&local_18);
        local_4 = 0xffffffff;
        FUN_0054f9f0(&local_18);
      }
      this_00 = (void *)((int)this_00 + 0xc);
    }
  }
  ExceptionList = local_c;
  return 1;
}

