
void __thiscall
FUN_005c1fc0(void *this,undefined4 param_1,undefined4 param_2,ushort param_3,undefined4 param_4,
            undefined4 param_5)

{
  void *this_00;
  int *piVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653aab;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this_00 = (void *)FUN_00618b70(0x50);
  local_4 = 0;
  if (this_00 == (void *)0x0) {
    piVar1 = (int *)0x0;
  }
  else {
    piVar1 = FUN_005d43c0(this_00,*(undefined4 *)((int)this + 0x138),param_1,param_2,param_3);
  }
  local_4 = 0xffffffff;
  if (piVar1 != (int *)0x0) {
    FUN_005f5440((void *)((int)this + 0x5d8),piVar1);
    (**(code **)(*piVar1 + 0x1c))(param_4,param_5);
  }
  ExceptionList = local_c;
  return;
}

