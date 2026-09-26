
void __fastcall FUN_0040f6d0(int param_1)

{
  int iVar1;
  void *pvVar2;
  undefined4 auStack_1c [2];
  int iStack_14;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062a413;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  if (*(undefined4 **)(param_1 + 0x18c) != (undefined4 *)0x0) {
    ExceptionList = &pvStack_c;
    (**(code **)**(undefined4 **)(param_1 + 0x18c))(1);
    *(undefined4 *)(param_1 + 0x18c) = 0;
  }
  if (*(int *)(param_1 + 0x184) != 0) {
    FUN_0060a790(auStack_1c,1);
    uStack_4 = 0;
    FUN_00609410(*(void **)(param_1 + 0x184),auStack_1c);
    if (iStack_14 != 0) {
      iVar1 = *(int *)(iStack_14 + 0x54);
      pvVar2 = (void *)FUN_00618b70(0x38);
      uStack_4 = CONCAT31(uStack_4._1_3_,1);
      if (pvVar2 == (void *)0x0) {
        pvVar2 = (void *)0x0;
      }
      else {
        pvVar2 = FUN_005f7a50(pvVar2,iVar1);
      }
      *(void **)(param_1 + 0x18c) = pvVar2;
    }
    uStack_4 = 0xffffffff;
    FUN_0060a810(auStack_1c);
  }
  ExceptionList = pvStack_c;
  return;
}

