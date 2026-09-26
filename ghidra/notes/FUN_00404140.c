
void __fastcall FUN_00404140(undefined4 *param_1)

{
  undefined4 *puVar1;
  int iVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00629668;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_00658520;
  local_4 = 0;
  if (param_1[0x18] != 0) {
    iVar2 = 0;
    if (0 < (int)param_1[0x12]) {
      do {
        puVar1 = *(undefined4 **)(param_1[0x18] + iVar2 * 4);
        if (puVar1 != (undefined4 *)0x0) {
          FUN_005fbfa0(puVar1);
          FUN_00618b60((undefined *)puVar1);
        }
        puVar1 = *(undefined4 **)(param_1[0x15] + iVar2 * 4);
        if (puVar1 != (undefined4 *)0x0) {
          FUN_005fbfa0(puVar1);
          FUN_00618b60((undefined *)puVar1);
        }
        iVar2 = iVar2 + 1;
      } while (iVar2 < (int)param_1[0x12]);
    }
    FUN_00618b60((undefined *)param_1[0x18]);
  }
  if ((undefined *)param_1[0x15] != (undefined *)0x0) {
    FUN_00618b60((undefined *)param_1[0x15]);
  }
  if ((undefined4 *)param_1[0x16] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x16])(3);
  }
  if ((undefined *)param_1[0x17] != (undefined *)0x0) {
    FUN_00618b60((undefined *)param_1[0x17]);
  }
  local_4 = 0xffffffff;
  FUN_00403e70(param_1);
  ExceptionList = pvStack_c;
  return;
}

