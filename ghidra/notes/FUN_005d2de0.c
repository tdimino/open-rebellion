
void __fastcall FUN_005d2de0(int *param_1)

{
  int *piVar1;
  int iVar2;
  void *pvVar3;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00654684;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = (int)&PTR_FUN_0066d198;
  param_1[7] = (int)&PTR_LAB_0066d188;
  local_4 = 2;
  FUN_005d3180(param_1);
  FUN_005d3ac0(param_1,(undefined1 *)0x1,0);
  FUN_005d3ac0(param_1,&DAT_00000002,0);
  FUN_005f5970((void *)(DAT_006bcbd4 + 0x5f4),param_1[6]);
  if (param_1[0x52] != 0) {
    FUN_005f5970((void *)(DAT_006bcbd4 + 0x5e4),param_1[6]);
    iVar2 = (**(code **)(*(int *)param_1[0x4a] + 0xa4))((int *)param_1[0x4a],param_1[0x52],0);
    FUN_005d8b00(iVar2);
  }
  piVar1 = (int *)param_1[0x14];
  if (piVar1 != (int *)0x0) {
    (**(code **)(*piVar1 + 8))(piVar1);
  }
  piVar1 = (int *)param_1[0x15];
  if (piVar1 != (int *)0x0) {
    (**(code **)(*piVar1 + 8))(piVar1);
  }
  piVar1 = (int *)param_1[0x16];
  if (piVar1 != (int *)0x0) {
    (**(code **)(*piVar1 + 8))(piVar1);
  }
  piVar1 = (int *)param_1[0x49];
  if (piVar1 != (int *)0x0) {
    (**(code **)(*piVar1 + 8))(piVar1);
  }
  pvVar3 = (void *)param_1[0x49];
  iVar2 = (**(code **)(*(int *)param_1[0xb] + 0x98))((int *)param_1[0xb],pvVar3,0);
  FUN_005d8b00(iVar2);
  piVar1 = (int *)param_1[0x4a];
  if (piVar1 != (int *)0x0) {
    (**(code **)(*piVar1 + 8))(piVar1);
  }
  iVar2 = (**(code **)(*(int *)param_1[0xb] + 0x98))((int *)param_1[0xb],param_1[0x4a],0);
  FUN_005d8b00(iVar2);
  FUN_005fbba0(param_1 + 0x42);
  FUN_005fbba0(param_1 + 0x3c);
  FUN_005d8a50(param_1);
  ExceptionList = pvVar3;
  return;
}

