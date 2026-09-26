
undefined4 __fastcall FUN_0052be30(void *param_1)

{
  uint *puVar1;
  int iVar2;
  undefined4 uVar3;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00644b70;
  local_c = ExceptionList;
  uVar3 = 0;
  ExceptionList = &local_c;
  local_10 = param_1;
  puVar1 = FUN_0040d760(param_1,&local_10);
  local_4 = 0;
  iVar2 = FUN_004ece60(puVar1);
  local_4 = 0xffffffff;
  FUN_00619730();
  if (iVar2 != 0) {
    puVar1 = FUN_0040d760(param_1,&local_10);
    local_4 = 1;
    uVar3 = FUN_00553400(puVar1);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return uVar3;
}

