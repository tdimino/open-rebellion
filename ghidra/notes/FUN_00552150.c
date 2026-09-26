
uint __thiscall FUN_00552150(void *this,int param_1,void *param_2)

{
  void *pvVar1;
  void *this_00;
  uint *puVar2;
  uint uVar3;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_2;
  puStack_8 = &LAB_00648d60;
  local_c = ExceptionList;
  uVar3 = 1;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  this_00 = (void *)thunk_FUN_005f5060(param_1);
  while( true ) {
    if (*(int *)((int)pvVar1 + 4) != -1) {
      ExceptionList = local_c;
      return uVar3;
    }
    if (this_00 == (void *)0x0) break;
    puVar2 = FUN_00403040(this_00,&param_2);
    local_4 = 1;
    uVar3 = FUN_00552000(this,puVar2,pvVar1);
    local_4 = 0xffffffff;
    FUN_00619730();
    this_00 = *(void **)((int)this_00 + 0x10);
    if (uVar3 == 0) {
      ExceptionList = local_c;
      return 0;
    }
  }
  ExceptionList = local_c;
  return uVar3;
}

