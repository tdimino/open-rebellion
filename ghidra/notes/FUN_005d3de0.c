
void __thiscall FUN_005d3de0(int param_1,undefined4 param_2,int param_3)

{
  void *pvVar1;
  uint uVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006546db;
  pvStack_c = ExceptionList;
  uVar2 = 2;
  if (param_3 != 2) {
    uVar2 = (uint)(*(int *)(param_1 + 0x3c) == 1);
  }
  ExceptionList = &pvStack_c;
  pvVar1 = (void *)FUN_00618b70(0x78);
  uStack_4 = 0;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_005ee590(pvVar1,*(undefined4 *)(param_1 + 0x30),param_1,param_2,uVar2);
  }
  uStack_4 = 0xffffffff;
  if (pvVar1 != (void *)0x0) {
    FUN_005f59f0((void *)(DAT_006bcbd4 + 0x5cc),(int)pvVar1 + 0x1c);
  }
  ExceptionList = pvStack_c;
  return;
}

