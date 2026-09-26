
int __thiscall FUN_0055b010(void *this,undefined4 param_1,undefined4 param_2,void *param_3)

{
  int iVar1;
  int iVar2;
  int local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649b78;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar1 = FUN_0053b0f0(this,param_1,param_2,param_3);
  FUN_0052e740(this,local_14);
  local_4 = 0;
  FUN_0051c9b0(local_14,0);
  FUN_0055a6e0(this,local_14,param_3);
  iVar2 = 0;
  if (iVar1 != 0) {
    iVar1 = FUN_0055aa80(this,param_3);
    iVar2 = 0;
    if (iVar1 != 0) {
      iVar2 = FUN_0055ab60(this,param_3);
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return iVar2;
}

