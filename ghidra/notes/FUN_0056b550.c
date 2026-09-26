
undefined4 __cdecl FUN_0056b550(int param_1,undefined4 param_2,void *param_3,undefined4 *param_4)

{
  bool bVar1;
  void *this;
  void *this_00;
  uint *puVar2;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this = param_3;
  puStack_8 = &LAB_0064b8c8;
  local_c = ExceptionList;
  local_1c = 1;
  local_18 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_1c);
  local_4 = 0xffffffff;
  FUN_00619730();
  *param_4 = 0;
  this_00 = (void *)thunk_FUN_005f5060(param_1);
  while ((this_00 != (void *)0x0 && (*(int *)((int)this + 4) == -1))) {
    puVar2 = FUN_00403040(this_00,&param_3);
    local_14 = 0x30;
    local_10 = 0x38;
    local_4 = 1;
    if ((*puVar2 >> 0x18 < 0x30) || (0x37 < *puVar2 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
    if (!bVar1) {
      local_1c = 0x40;
      local_18 = 6;
      local_4 = 2;
      FUN_00520580(this,&local_1c);
      local_4 = 0xffffffff;
      FUN_00619730();
      *param_4 = 0xffffffff;
    }
    this_00 = *(void **)((int)this_00 + 0x10);
  }
  ExceptionList = local_c;
  return 1;
}

