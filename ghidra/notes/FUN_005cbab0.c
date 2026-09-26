
void __thiscall FUN_005cbab0(void *this,void *param_1)

{
  int iVar1;
  void *pvVar2;
  int iStack_18;
  uint uStack_14;
  undefined4 *puStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006542c6;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  (**(code **)(*(int *)this + 4))();
  iStack_18 = 0;
  uStack_14 = 0;
  FUN_005f4d90(param_1,&iStack_18);
  while (iStack_18 != 0) {
    iStack_18 = iStack_18 + -1;
    FUN_005f4d90(param_1,&uStack_14);
    if (DAT_006bc470 == (undefined4 *)0x0) {
      puStack_10 = (undefined4 *)FUN_00618b70(0x48);
      uStack_4 = 0;
      if (puStack_10 == (undefined4 *)0x0) {
        DAT_006bc470 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc470 = FUN_005a6290(puStack_10);
      }
      uStack_4 = 0xffffffff;
    }
    iVar1 = FUN_005a6480(DAT_006bc470,uStack_14);
    if (iVar1 != 0) {
      puStack_10 = (undefined4 *)FUN_00618b70(0x20);
      uStack_4 = 1;
      if (puStack_10 == (undefined4 *)0x0) {
        pvVar2 = (void *)0x0;
      }
      else {
        pvVar2 = FUN_005ca350(puStack_10,iVar1);
      }
      uStack_4 = 0xffffffff;
      FUN_005f5440(this,pvVar2);
    }
  }
  ExceptionList = pvStack_c;
  return;
}

