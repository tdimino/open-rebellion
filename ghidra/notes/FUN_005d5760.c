
void __fastcall FUN_005d5760(int *param_1)

{
  int *piVar1;
  int iVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006547b8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = (int)&PTR_FUN_0066d278;
  param_1[7] = (int)&PTR_LAB_0066d268;
  local_4 = 0;
  if ((*(int *)(DAT_006bcbd4 + 0x680) != 0) && (param_1[0x17] != 0)) {
    iVar2 = (**(code **)(*(int *)param_1[0x15] + 0xa4))((int *)param_1[0x15],param_1[0x17],0);
    FUN_005d8b00(iVar2);
  }
  if (param_1[0x19] != 0) {
    FUN_005d5950(param_1);
    (**(code **)(*(int *)param_1[0x13] + 8))((int *)param_1[0x13]);
    (**(code **)(*(int *)param_1[0x14] + 8))((int *)param_1[0x14]);
    iVar2 = (**(code **)(*(int *)param_1[0xb] + 0x98))((int *)param_1[0xb],param_1[0x14],0);
    FUN_005d8b00(iVar2);
  }
  if (*(int *)(DAT_006bcbd4 + 0x680) != 0) {
    (**(code **)(*(int *)param_1[0x15] + 8))((int *)param_1[0x15]);
    iVar2 = (**(code **)(*(int *)param_1[0xb] + 0x98))((int *)param_1[0xb],param_1[0x15],0);
    FUN_005d8b00(iVar2);
  }
  FUN_005d5870((int)param_1);
  piVar1 = (int *)param_1[0xf];
  if (piVar1 != (int *)0x0) {
    (**(code **)(*piVar1 + 8))(piVar1);
  }
  piVar1 = (int *)param_1[0x10];
  if (piVar1 != (int *)0x0) {
    (**(code **)(*piVar1 + 8))(piVar1);
  }
  local_4 = 0xffffffff;
  FUN_005d8a50(param_1);
  ExceptionList = pvStack_c;
  return;
}

