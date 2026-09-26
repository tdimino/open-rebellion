
void __thiscall FUN_005cd8e0(void *this,void *param_1)

{
  undefined4 *puVar1;
  int iVar2;
  int iStack_14;
  uint uStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0065438b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  (**(code **)(*(int *)this + 4))();
  iStack_14 = 0;
  uStack_10 = 0;
  FUN_005f4d90(param_1,&iStack_14);
  while (iStack_14 != 0) {
    iStack_14 = iStack_14 + -1;
    FUN_005f4d90(param_1,&uStack_10);
    if (DAT_006bc470 == (undefined4 *)0x0) {
      puVar1 = (undefined4 *)FUN_00618b70(0x48);
      uStack_4 = 0;
      if (puVar1 == (undefined4 *)0x0) {
        DAT_006bc470 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc470 = FUN_005a6290(puVar1);
      }
      uStack_4 = 0xffffffff;
    }
    iVar2 = FUN_005a6340(DAT_006bc470,uStack_10);
    if (iVar2 != 0) {
      puVar1 = (undefined4 *)FUN_00618b70(0x1c);
      if (puVar1 == (undefined4 *)0x0) {
        puVar1 = (undefined4 *)0x0;
      }
      else {
        *puVar1 = &PTR_FUN_0066d04c;
        puVar1[1] = 0;
        puVar1[2] = 0;
        puVar1[3] = 0;
        puVar1[4] = 0;
        *(undefined2 *)(puVar1 + 5) = 0;
        puVar1[6] = iVar2;
      }
      FUN_005ceaa0(this,puVar1);
    }
  }
  ExceptionList = pvStack_c;
  return;
}

