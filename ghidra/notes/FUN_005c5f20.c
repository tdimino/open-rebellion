
uint __thiscall FUN_005c5f20(void *this,int param_1)

{
  int iVar1;
  void *pvVar2;
  uint uVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653c7b;
  local_c = ExceptionList;
  if (param_1 != 0) {
    ExceptionList = &local_c;
    iVar1 = FUN_005ca4b0(this,*(uint *)(param_1 + 0xec));
    if (iVar1 == 0) {
      pvVar2 = (void *)FUN_00618b70(0x20);
      local_4 = 0;
      if (pvVar2 == (void *)0x0) {
        pvVar2 = (void *)0x0;
      }
      else {
        pvVar2 = FUN_005e9cc0(pvVar2,param_1);
      }
      local_4 = 0xffffffff;
      uVar3 = FUN_005f5440(this,pvVar2);
      ExceptionList = local_c;
      return uVar3;
    }
  }
  ExceptionList = local_c;
  return 0;
}

