
void __thiscall FUN_00600a40(void *this,void *param_1,undefined4 param_2)

{
  int iVar1;
  char *pcVar2;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00656038;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f2fc0(local_18,param_2);
  local_4 = 0;
  iVar1 = FUN_00583c40((int)local_18);
  if (iVar1 != 0) {
    pcVar2 = (char *)FUN_00583c40((int)local_18);
    FUN_00600970(this,param_1,pcVar2);
  }
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  ExceptionList = local_c;
  return;
}

