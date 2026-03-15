
void __cdecl FUN_00435a50(undefined4 param_1,int param_2)

{
  void *this;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d44b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00435b60();
  this = (void *)FUN_00618b70(0x34);
  local_4 = 0;
  if (this == (void *)0x0) {
    DAT_006b14e4 = (void *)0x0;
  }
  else {
    DAT_006b14e4 = FUN_00435bc0(this,param_1,param_2);
  }
  local_4 = 0xffffffff;
  if ((DAT_006b14e4 != (void *)0x0) && (*(int *)((int)DAT_006b14e4 + 0x20) == 0)) {
    FUN_00435b60();
  }
  ExceptionList = local_c;
  return;
}

