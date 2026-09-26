
void __fastcall FUN_0040f790(int param_1)

{
  int iVar1;
  void *pvVar2;
  undefined4 auStack_1c [2];
  int iStack_14;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062a433;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  if (*(undefined4 **)(param_1 + 400) != (undefined4 *)0x0) {
    ExceptionList = &pvStack_c;
    (**(code **)**(undefined4 **)(param_1 + 400))(1);
    *(undefined4 *)(param_1 + 400) = 0;
  }
  if (*(int *)(param_1 + 0x188) != 0) {
    FUN_0060a790(auStack_1c,1);
    uStack_4 = 0;
    FUN_00609410(*(void **)(param_1 + 0x188),auStack_1c);
    if (iStack_14 != 0) {
      iVar1 = *(int *)(iStack_14 + 0x54);
      pvVar2 = (void *)FUN_00618b70(0x38);
      uStack_4 = CONCAT31(uStack_4._1_3_,1);
      if (pvVar2 == (void *)0x0) {
        pvVar2 = (void *)0x0;
      }
      else {
        pvVar2 = FUN_005f7be0(pvVar2,iVar1);
      }
      *(void **)(param_1 + 400) = pvVar2;
    }
    uStack_4 = 0xffffffff;
    FUN_0060a810(auStack_1c);
  }
  ExceptionList = pvStack_c;
  return;
}

