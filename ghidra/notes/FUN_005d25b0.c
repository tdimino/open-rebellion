
uint __thiscall FUN_005d25b0(void *this,int param_1)

{
  int iVar1;
  void *pvVar2;
  uint uVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065461b;
  local_c = ExceptionList;
  if (param_1 != 0) {
    ExceptionList = &local_c;
    iVar1 = FUN_005d2650(this,*(int *)(param_1 + 0xec));
    if (iVar1 == 0) {
      pvVar2 = (void *)FUN_00618b70(0x14);
      local_4 = 0;
      if (pvVar2 == (void *)0x0) {
        pvVar2 = (void *)0x0;
      }
      else {
        pvVar2 = FUN_005d0c50(pvVar2,param_1);
      }
      local_4 = 0xffffffff;
      uVar3 = FUN_005f58d0(this,(int)pvVar2);
      ExceptionList = local_c;
      return uVar3;
    }
  }
  ExceptionList = local_c;
  return 0;
}

