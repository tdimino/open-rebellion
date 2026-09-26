
void __fastcall FUN_005eeaf0(undefined4 *param_1)

{
  int iVar1;
  undefined1 *puVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00655473;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066db48;
  param_1[7] = &PTR_LAB_0066db38;
  puVar2 = &LAB_005eeb90;
  local_4 = 1;
  iVar1 = (**(code **)(*(int *)param_1[0xb] + 0xa0))((int *)param_1[0xb],&LAB_005eeb90,param_1,0);
  FUN_005d8b00(iVar1);
  iVar1 = (**(code **)(*(int *)param_1[0xb] + 0xa4))((int *)param_1[0xb],param_1[0xe],0);
  FUN_005d8b00(iVar1);
  FUN_005fbba0(param_1 + 0x10);
  FUN_005d8a50(param_1);
  ExceptionList = puVar2;
  return;
}

