
void __fastcall FUN_005e1200(undefined4 *param_1)

{
  undefined4 *puVar1;
  undefined *puVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00654d58;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066d670;
  puVar1 = (undefined4 *)param_1[0x28];
  local_4 = 0;
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar2 = (undefined *)param_1[0x29];
  if (puVar2 != (undefined *)0x0) {
    FUN_00601990((int)puVar2);
    FUN_00618b60(puVar2);
  }
  puVar2 = (undefined *)param_1[0x2a];
  if (puVar2 != (undefined *)0x0) {
    FUN_00601990((int)puVar2);
    FUN_00618b60(puVar2);
  }
  puVar2 = (undefined *)param_1[0x2b];
  if (puVar2 != (undefined *)0x0) {
    FUN_00601990((int)puVar2);
    FUN_00618b60(puVar2);
  }
  puVar2 = (undefined *)param_1[0x2c];
  if (puVar2 != (undefined *)0x0) {
    FUN_00601990((int)puVar2);
    FUN_00618b60(puVar2);
  }
  puVar2 = (undefined *)param_1[0x2d];
  if (puVar2 != (undefined *)0x0) {
    FUN_00601990((int)puVar2);
    FUN_00618b60(puVar2);
  }
  puVar2 = (undefined *)param_1[0x2e];
  if (puVar2 != (undefined *)0x0) {
    FUN_00601990((int)puVar2);
    FUN_00618b60(puVar2);
  }
  puVar2 = (undefined *)param_1[0x2f];
  if (puVar2 != (undefined *)0x0) {
    FUN_00601990((int)puVar2);
    FUN_00618b60(puVar2);
  }
  local_4 = 0xffffffff;
  FUN_005ef0e0(param_1);
  ExceptionList = pvStack_c;
  return;
}

