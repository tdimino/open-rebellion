
void * __thiscall FUN_004ed270(void *this,void *param_1)

{
  int iVar1;
  undefined4 *puVar2;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063ed06;
  local_c = ExceptionList;
  iVar1 = *(int *)((int)this + 0x54);
  if (iVar1 == 0) {
    ExceptionList = &local_c;
    puVar2 = (undefined4 *)FUN_004ece30(&local_10);
    local_4 = 1;
  }
  else {
    puVar2 = (undefined4 *)(iVar1 + 0x30);
    ExceptionList = &local_c;
  }
  FUN_004f26d0(param_1,puVar2);
  local_4 = 0;
  if (iVar1 == 0) {
    FUN_00619730();
  }
  ExceptionList = local_c;
  return param_1;
}

