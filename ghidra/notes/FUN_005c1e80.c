
void __thiscall
FUN_005c1e80(void *this,undefined4 param_1,undefined4 *param_2,undefined4 param_3,undefined4 param_4
            )

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653a6b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar1 = (void *)FUN_00618b70(0x44);
  local_4 = 0;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_005d90b0(pvVar1,*(undefined4 *)((int)this + 0x138),param_1,param_2,param_3,param_4)
    ;
  }
  local_4 = 0xffffffff;
  if (pvVar1 != (void *)0x0) {
    FUN_005f5440((void *)((int)this + 0x5d8),pvVar1);
  }
  ExceptionList = local_c;
  return;
}

