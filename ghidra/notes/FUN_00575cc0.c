
undefined4 __cdecl FUN_00575cc0(int param_1,undefined4 param_2,void *param_3,undefined4 *param_4)

{
  int iVar1;
  void *this;
  void *this_00;
  int *piVar2;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this = param_3;
  puStack_8 = &LAB_0064c9e8;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  *param_4 = 0;
  this_00 = (void *)thunk_FUN_005f5060(param_1);
  while ((this_00 != (void *)0x0 && (*(int *)((int)this + 4) == -1))) {
    piVar2 = FUN_00559950(this_00,&param_1);
    param_3 = (void *)0x3c000009;
    iVar1 = *piVar2;
    local_4 = 1;
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
    if (iVar1 == 0x3c000009) {
      local_14 = 0x40;
      local_10 = 0xb;
      local_4 = 2;
      FUN_00520580(this,&local_14);
      local_4 = 0xffffffff;
      FUN_00619730();
      *param_4 = 0xffffffff;
    }
    this_00 = *(void **)((int)this_00 + 0x10);
  }
  ExceptionList = local_c;
  return 1;
}

