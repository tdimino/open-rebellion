
void __fastcall FUN_005da780(undefined4 *param_1)

{
  int iVar1;
  void *pvVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00654b73;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066d500;
  param_1[7] = &PTR_LAB_0066d4f0;
  local_4 = 1;
  *(undefined4 *)(DAT_006bcbd4 + 0x654) = 0;
  pvVar2 = (void *)param_1[0xe];
  iVar1 = (**(code **)(*(int *)param_1[0xb] + 0xa4))((int *)param_1[0xb],pvVar2,0);
  FUN_005d8b00(iVar1);
  iVar1 = (**(code **)(*(int *)param_1[0xb] + 0xa4))((int *)param_1[0xb],param_1[0xf],0);
  FUN_005d8b00(iVar1);
  FUN_005fbba0(param_1 + 0x10);
  FUN_005d8a50(param_1);
  ExceptionList = pvVar2;
  return;
}

