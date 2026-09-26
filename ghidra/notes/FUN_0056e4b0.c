
undefined4 __cdecl FUN_0056e4b0(int param_1,undefined4 param_2,void *param_3,undefined4 *param_4)

{
  uint uVar1;
  bool bVar2;
  void *this;
  void *this_00;
  uint *puVar3;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this = param_3;
  puStack_8 = &LAB_0064bd98;
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
  do {
    if ((this_00 == (void *)0x0) || (*(int *)((int)this + 4) != -1)) {
      ExceptionList = local_c;
      return 1;
    }
    puVar3 = FUN_00559950(this_00,&local_1c);
    local_4 = 1;
    if ((*puVar3 & 0xffffff) == 0x242) {
LAB_0056e5a4:
      bVar2 = false;
    }
    else {
      puVar3 = FUN_00559950(this_00,&param_3);
      uVar1 = *puVar3;
      FUN_00619730();
      if ((uVar1 & 0xffffff) == 0x241) goto LAB_0056e5a4;
      puVar3 = FUN_00559950(this_00,&param_1);
      uVar1 = *puVar3;
      FUN_00619730();
      if ((uVar1 & 0xffffff) == 0x343) goto LAB_0056e5a4;
      bVar2 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar2) {
      local_14 = 0x40;
      local_10 = 10;
      local_4 = 2;
      FUN_00520580(this,&local_14);
      local_4 = 0xffffffff;
      FUN_00619730();
      *param_4 = 0xffffffff;
    }
    this_00 = *(void **)((int)this_00 + 0x10);
  } while( true );
}

