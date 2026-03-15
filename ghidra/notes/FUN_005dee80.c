
void __fastcall FUN_005dee80(undefined4 *param_1)

{
  undefined4 *puVar1;
  int iVar2;
  undefined4 *puVar3;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00654cc8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066d608;
  local_4 = 0;
  puVar3 = param_1 + 0x2b;
  iVar2 = 5;
  do {
    puVar1 = (undefined4 *)*puVar3;
    if (puVar1 != (undefined4 *)0x0) {
      FUN_005fbfa0(puVar1);
      FUN_00618b60((undefined *)puVar1);
    }
    puVar3 = puVar3 + 1;
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  local_4 = 0xffffffff;
  FUN_005ef0e0(param_1);
  ExceptionList = pvStack_c;
  return;
}

