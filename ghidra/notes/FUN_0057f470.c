
int __thiscall FUN_0057f470(void *this,void *param_1)

{
  int iVar1;
  int *local_18;
  uint local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0064e670;
  local_c = ExceptionList;
  iVar1 = 1;
  local_18 = (int *)0x0;
  local_14[0] = 1;
  local_14[1] = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_1,local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  if (*(int *)((int)param_1 + 4) == -1) {
    local_14[1] = 0x12;
    local_14[0] = 1;
    local_4 = 1;
    iVar1 = FUN_005057f0((uint *)((int)this + 0x3c),&local_18,local_14,param_1);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  if ((iVar1 != 0) && (*(int *)((int)param_1 + 4) == -1)) {
    iVar1 = (**(code **)(*local_18 + 0x1bc))((int)this + 0x40,param_1);
  }
  ExceptionList = local_c;
  return iVar1;
}

