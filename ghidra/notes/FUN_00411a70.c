
void __fastcall FUN_00411a70(undefined4 *param_1)

{
  undefined4 *puVar1;
  int iVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0062a708;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_006587e8;
  local_4 = 0;
  if (param_1[0x14] != 0) {
    iVar2 = 0;
    if (0 < (int)param_1[0x13]) {
      do {
        puVar1 = *(undefined4 **)(param_1[0x14] + iVar2 * 4);
        if (puVar1 != (undefined4 *)0x0) {
          FUN_005fbfa0(puVar1);
          FUN_00618b60((undefined *)puVar1);
        }
        iVar2 = iVar2 + 1;
      } while (iVar2 < (int)param_1[0x13]);
    }
    FUN_00618b60((undefined *)param_1[0x14]);
  }
  local_4 = 0xffffffff;
  FUN_00403e70(param_1);
  ExceptionList = pvStack_c;
  return;
}

