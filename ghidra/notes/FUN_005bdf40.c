
void __thiscall FUN_005bdf40(void *this,void *param_1)

{
  undefined4 *puVar1;
  int local_18;
  void *local_14;
  undefined4 *puStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006536c9;
  pvStack_c = ExceptionList;
  local_18 = 0;
  ExceptionList = &pvStack_c;
  local_14 = this;
  (**(code **)(*(int *)this + 4))();
  FUN_005f4d90(param_1,&local_18);
  while (local_18 != 0) {
    local_18 = local_18 + -1;
    puVar1 = (undefined4 *)FUN_00618b70(0x4c);
    uStack_4 = 0;
    puStack_10 = puVar1;
    if (puVar1 == (undefined4 *)0x0) {
      puVar1 = (undefined4 *)0x0;
    }
    else {
      FUN_005a6210(puVar1);
      puVar1[7] = &PTR_FUN_0066bacc;
      puVar1[8] = 0;
      puVar1[9] = &PTR_FUN_0066bac8;
      puVar1[10] = 0;
      uStack_4 = CONCAT31(uStack_4._1_3_,3);
      FUN_005f2f50(puVar1 + 0xe);
      *puVar1 = &PTR_FUN_0066bad0;
    }
    uStack_4 = 0xffffffff;
    if (puVar1 != (undefined4 *)0x0) {
      FUN_005bddb0(puVar1,param_1);
      FUN_005f5440(local_14,puVar1);
    }
  }
  ExceptionList = pvStack_c;
  return;
}

