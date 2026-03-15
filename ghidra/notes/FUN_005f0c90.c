
void __fastcall FUN_005f0c90(undefined4 *param_1)

{
  undefined *puVar1;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006554c8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066dc30;
  puVar1 = (undefined *)param_1[0x27];
  local_4 = 0;
  if (puVar1 != (undefined *)0x0) {
    FUN_00601990((int)puVar1);
    FUN_00618b60(puVar1);
  }
  puVar1 = (undefined *)param_1[0x28];
  if (puVar1 != (undefined *)0x0) {
    FUN_00601990((int)puVar1);
    FUN_00618b60(puVar1);
  }
  local_4 = 0xffffffff;
  FUN_005ef0e0(param_1);
  ExceptionList = pvStack_c;
  return;
}

