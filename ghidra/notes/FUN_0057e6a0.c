
int __thiscall FUN_0057e6a0(void *this,int *param_1)

{
  int *this_00;
  int iVar1;
  uint local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_1;
  puStack_8 = &LAB_0064e3d8;
  local_c = ExceptionList;
  iVar1 = 1;
  local_14[0] = 1;
  local_14[1] = 1;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_1,local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  param_1 = (int *)0x0;
  local_14[0] = 1;
  local_14[1] = 0xffffffff;
  local_4 = 1;
  FUN_00520580(this_00,local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  if (this_00[1] == -1) {
    local_14[0] = 1;
    local_14[1] = 0x12;
    local_4 = 2;
    iVar1 = FUN_005057f0((uint *)((int)this + 0x3c),&param_1,local_14,this_00);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  if ((iVar1 != 0) && (this_00[1] == -1)) {
    iVar1 = (**(code **)(*param_1 + 0x1c4))((int)this + 0x40,this_00);
  }
  ExceptionList = local_c;
  return iVar1;
}

