
void FUN_005ad430(undefined4 param_1,undefined1 param_2,undefined4 param_3)

{
  void *this;
  longlong lVar1;
  uint *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00652e9b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this = (void *)FUN_00618b70(0x220);
  local_4 = 0;
  if (this != (void *)0x0) {
    puVar2 = (uint *)0x0;
    lVar1 = __ftol();
    FUN_005ce500(this,DAT_0066c22c,param_2,param_3,(int)lVar1,puVar2);
  }
  ExceptionList = local_c;
  return;
}

