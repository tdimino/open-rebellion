
void __fastcall FUN_004121d0(undefined4 *param_1)

{
  undefined4 *puVar1;
  int iVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0062a7c8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_00658848;
  local_4 = 0;
  if (param_1[0x13] != 0) {
    iVar2 = 0;
    if (0 < (int)param_1[0x12]) {
      do {
        puVar1 = *(undefined4 **)(param_1[0x13] + iVar2 * 4);
        if (puVar1 != (undefined4 *)0x0) {
          FUN_005fbfa0(puVar1);
          FUN_00618b60((undefined *)puVar1);
        }
        iVar2 = iVar2 + 1;
      } while (iVar2 < (int)param_1[0x12]);
    }
    FUN_00618b60((undefined *)param_1[0x13]);
  }
  local_4 = 0xffffffff;
  FUN_00403e70(param_1);
  ExceptionList = pvStack_c;
  return;
}

