
undefined4 __thiscall FUN_005f6790(void *this,undefined4 *param_1)

{
  undefined4 *puVar1;
  int iVar2;
  undefined4 uVar3;
  undefined4 local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006557d8;
  pvStack_c = ExceptionList;
  uVar3 = 0;
  ExceptionList = &pvStack_c;
  FUN_005f66c0(local_14);
  local_4 = 0;
  puVar1 = (undefined4 *)FUN_005f6730((int)this);
  do {
    if (puVar1 == (undefined4 *)0x0) {
LAB_005f67f7:
      iVar2 = FUN_005f6730((int)local_14);
      while (iVar2 != 0) {
        FUN_005f6700(this,iVar2);
        iVar2 = FUN_005f6730((int)local_14);
      }
      local_4 = 0xffffffff;
      FUN_005f66f0(local_14);
      ExceptionList = pvStack_c;
      return uVar3;
    }
    if (puVar1 == param_1) {
      if (puVar1 != (undefined4 *)0x0) {
        (**(code **)*puVar1)(1);
      }
      uVar3 = 1;
      goto LAB_005f67f7;
    }
    FUN_005f6700(local_14,(int)puVar1);
    puVar1 = (undefined4 *)FUN_005f6730((int)this);
  } while( true );
}

