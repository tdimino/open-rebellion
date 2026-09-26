
int __thiscall FUN_00552a50(void *this,int param_1,int *param_2,void *param_3)

{
  int *piVar1;
  void *this_00;
  uint *puVar2;
  int iVar3;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar1 = param_2;
  puStack_8 = &LAB_00648e70;
  local_c = ExceptionList;
  iVar3 = 1;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  this_00 = (void *)thunk_FUN_005f5060(param_1);
  while( true ) {
    if (piVar1[1] != -1) {
      ExceptionList = local_c;
      return iVar3;
    }
    if (this_00 == (void *)0x0) break;
    puVar2 = FUN_00403040(this_00,&param_2);
    local_4 = 1;
    iVar3 = FUN_005529a0(this,puVar2,piVar1,param_3);
    local_4 = 0xffffffff;
    FUN_00619730();
    this_00 = *(void **)((int)this_00 + 0x10);
    if (iVar3 == 0) {
      ExceptionList = local_c;
      return 0;
    }
  }
  ExceptionList = local_c;
  return iVar3;
}

