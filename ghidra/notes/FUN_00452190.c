
void __fastcall FUN_00452190(undefined4 *param_1)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0062fcd8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_00659de0;
  local_4 = 0;
  if ((undefined4 *)param_1[0x51] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x51])(1);
  }
  puVar1 = (undefined4 *)param_1[0x52];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  if ((undefined *)param_1[0x50] != (undefined *)0x0) {
    FUN_00618b60((undefined *)param_1[0x50]);
  }
  puVar1 = (undefined4 *)param_1[0x29];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  local_4 = 0xffffffff;
  FUN_00606580(param_1);
  ExceptionList = local_c;
  return;
}

